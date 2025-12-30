// Generated macro for map_result_ptr (function)
macro_rules! Depcrate_tlsmap_result_ptr {
() => {
// Module: crate::tls
// Provides: {"map_result_ptr"}
// Dependencies: {}
pub fn map_result_ptr < 'a , T > (bssl_result : * const T) -> Result < & 'a T > { match unsafe { bssl_result . as_ref () } { Some (v) => Ok (v) , None => Err (Error :: TlsFail) , } }
};
}
