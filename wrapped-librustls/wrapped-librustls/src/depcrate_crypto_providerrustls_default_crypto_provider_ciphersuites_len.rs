// Generated macro for rustls_default_crypto_provider_ciphersuites_len (function)
macro_rules! Depcrate_crypto_providerrustls_default_crypto_provider_ciphersuites_len {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_default_crypto_provider_ciphersuites_len"}
// Dependencies: {}
# [doc = " Returns the number of ciphersuites the default process-wide crypto provider supports."] # [doc = ""] # [doc = " You can use this to know the maximum allowed index for use with"] # [doc = " `rustls_default_crypto_provider_ciphersuites_get`."] # [doc = ""] # [doc = " This function will return 0 if no process-wide default `rustls_crypto_provider` is available."] # [no_mangle] pub extern "C" fn rustls_default_crypto_provider_ciphersuites_len () -> usize { ffi_panic_boundary ! { match get_default_or_install_from_crate_features () { Some (provider) => provider . cipher_suites . len () , None => return 0 , } } }
};
}
