// Generated macro for solana_rpc_client_api (module)
macro_rules! Depcratesolana_rpc_client_api {
() => {
// Module: crate
// Provides: {"solana_rpc_client_api"}
// Dependencies: {}
pub mod solana_rpc_client_api { pub mod client_error { # [derive (thiserror :: Error , Debug)] # [error ("mock-error")] pub struct ClientError ; pub type Result < T > = std :: result :: Result < T , ClientError > ; } }
};
}
