// Generated macro for solana_rpc_client_nonce_utils (module)
macro_rules! Depcratesolana_rpc_client_nonce_utils {
() => {
// Module: crate
// Provides: {"solana_rpc_client_nonce_utils"}
// Dependencies: {}
pub mod solana_rpc_client_nonce_utils { use { super :: solana_sdk :: { account :: ReadableAccount , account_utils :: StateMut , pubkey :: Pubkey } , solana_nonce :: { state :: { Data , DurableNonce } , versions :: Versions , } , } ; # [derive (thiserror :: Error , Debug)] # [error ("mock-error")] pub struct Error ; pub fn data_from_account < T : ReadableAccount + StateMut < Versions > > (_account : & T ,) -> Result < Data , Error > { Ok (Data :: new (Pubkey :: new_unique () , DurableNonce :: default () , 5000 ,)) } }
};
}
