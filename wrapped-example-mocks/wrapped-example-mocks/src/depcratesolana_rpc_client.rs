// Generated macro for solana_rpc_client (module)
macro_rules! Depcratesolana_rpc_client {
() => {
// Module: crate
// Provides: {"solana_rpc_client"}
// Dependencies: {}
pub mod solana_rpc_client { pub mod rpc_client { use { super :: super :: { solana_rpc_client_api :: client_error :: Result as ClientResult , solana_sdk :: { account :: Account , hash :: Hash , pubkey :: Pubkey , signature :: Signature , transaction :: Transaction , } , } , std :: { cell :: RefCell , collections :: HashMap , rc :: Rc } , } ; # [derive (Default)] pub struct RpcClient { get_account_responses : Rc < RefCell < HashMap < Pubkey , Account > > > , } impl RpcClient { pub fn new (_url : String) -> Self { RpcClient :: default () } pub fn get_latest_blockhash (& self) -> ClientResult < Hash > { Ok (Hash :: default ()) } pub fn send_and_confirm_transaction (& self , _transaction : & Transaction ,) -> ClientResult < Signature > { Ok (Signature) } pub fn get_minimum_balance_for_rent_exemption (& self , _data_len : usize ,) -> ClientResult < u64 > { Ok (0) } pub fn get_account (& self , pubkey : & Pubkey) -> ClientResult < Account > { Ok (self . get_account_responses . borrow () . get (pubkey) . cloned () . unwrap ()) } pub fn set_get_account_response (& self , pubkey : Pubkey , account : Account) { self . get_account_responses . borrow_mut () . insert (pubkey , account) ; } pub fn get_balance (& self , _pubkey : & Pubkey) -> ClientResult < u64 > { Ok (0) } } } }
};
}
