// Generated macro for map_result_zero_is_success (function)
macro_rules! Depcrate_tlsmap_result_zero_is_success {
() => {
// Module: crate::tls
// Provides: {"map_result_zero_is_success"}
// Dependencies: {}
pub fn map_result_zero_is_success (bssl_result : c_int) -> Result < () > { match bssl_result { 0 => Ok (()) , _ => Err (Error :: TlsFail) , } }
};
}
