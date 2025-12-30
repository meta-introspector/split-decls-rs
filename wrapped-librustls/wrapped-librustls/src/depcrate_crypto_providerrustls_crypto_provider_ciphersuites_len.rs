// Generated macro for rustls_crypto_provider_ciphersuites_len (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_ciphersuites_len {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_ciphersuites_len"}
// Dependencies: {}
# [doc = " Returns the number of ciphersuites the `rustls_crypto_provider` supports."] # [doc = ""] # [doc = " You can use this to know the maximum allowed index for use with"] # [doc = " `rustls_crypto_provider_ciphersuites_get`."] # [doc = ""] # [doc = " This function will return 0 if the `provider` is NULL."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_ciphersuites_len (provider : * const rustls_crypto_provider ,) -> usize { ffi_panic_boundary ! { try_clone_arc ! (provider) . cipher_suites . len () } }
};
}
