// Generated macro for map_result (function)
macro_rules! Depcrate_tlsmap_result {
() => {
// Module: crate::tls
// Provides: {"map_result"}
// Dependencies: {}
pub fn map_result (bssl_result : c_int) -> Result < () > { match bssl_result { 1 => Ok (()) , _ => Err (Error :: TlsFail) , } }
};
}
