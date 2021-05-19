use async_trait::async_trait;
use grpc_interfaces::txpool::{
    txpool_control_client::TxpoolControlClient, AccountInfoReply, AccountInfoRequest,
};
use interfaces::world_state::{AccountInfo, WorldState};
use reth_core::H256;
use tonic::{transport::Channel, transport::Server, Request, Response, Status};
use tokio::sync::Mutex;

pub struct GrpcWorldState {
    client: Mutex<TxpoolControlClient<Channel>>,
}

impl GrpcWorldState {
    pub async fn new(address: String) -> GrpcWorldState {
        //let addr = "[::1]:50051".parse().unwrap();
        let client = Mutex::new(TxpoolControlClient::connect(address).await.unwrap());
        GrpcWorldState { client }
    }
}

#[async_trait]
impl WorldState for GrpcWorldState {
    async fn account_info(
        &self,
        block_id: reth_core::BlockId,
        account: reth_core::Address,
    ) -> Option<AccountInfo> {
        let response = self
            .client.lock().await
            .account_info(AccountInfoRequest {
                block_hash: Some(H256::zero().into()),
                account: Some(account.into()),
            })
            .await
            .ok()?;

        Some(AccountInfo {
            balance: response.get_ref().balance.clone().unwrap().into(),
            nonce: response.get_ref().nonce,
        })
    }
}