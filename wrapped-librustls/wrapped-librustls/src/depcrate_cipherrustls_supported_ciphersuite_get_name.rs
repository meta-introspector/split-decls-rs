// Generated macro for rustls_supported_ciphersuite_get_name (function)
macro_rules! Depcrate_cipherrustls_supported_ciphersuite_get_name {
() => {
// Module: crate::cipher
// Provides: {"rustls_supported_ciphersuite_get_name"}
// Dependencies: {}
# [doc = " Returns the name of the ciphersuite as a `rustls_str`."] # [doc = ""] # [doc = " If the provided ciphersuite is invalid, the `rustls_str` will contain the"] # [doc = " empty string. The lifetime of the `rustls_str` is the lifetime of the program,"] # [doc = " it does not need to be freed."] # [no_mangle] pub extern "C" fn rustls_supported_ciphersuite_get_name (supported_ciphersuite : * const rustls_supported_ciphersuite ,) -> rustls_str < 'static > { let supported_ciphersuite = try_ref_from_ptr ! (supported_ciphersuite) ; let s = supported_ciphersuite . suite () . as_str () . unwrap_or ("") ; match rustls_str :: try_from (s) { Ok (s) => s , Err (_) => rustls_str :: from_str_unchecked ("") , } }
};
}
