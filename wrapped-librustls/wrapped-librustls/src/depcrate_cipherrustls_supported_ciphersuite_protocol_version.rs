// Generated macro for rustls_supported_ciphersuite_protocol_version (function)
macro_rules! Depcrate_cipherrustls_supported_ciphersuite_protocol_version {
() => {
// Module: crate::cipher
// Provides: {"rustls_supported_ciphersuite_protocol_version"}
// Dependencies: {}
# [doc = " Returns the `rustls_tls_version` of the ciphersuite."] # [doc = ""] # [doc = " See also `RUSTLS_ALL_VERSIONS`."] # [no_mangle] pub extern "C" fn rustls_supported_ciphersuite_protocol_version (supported_ciphersuite : * const rustls_supported_ciphersuite ,) -> rustls_tls_version { ffi_panic_boundary ! { rustls_tls_version :: from (try_ref_from_ptr ! (supported_ciphersuite) . version ()) } }
};
}
