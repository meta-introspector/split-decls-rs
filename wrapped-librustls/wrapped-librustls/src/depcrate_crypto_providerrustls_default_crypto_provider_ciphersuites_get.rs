// Generated macro for rustls_default_crypto_provider_ciphersuites_get (function)
macro_rules! Depcrate_crypto_providerrustls_default_crypto_provider_ciphersuites_get {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_default_crypto_provider_ciphersuites_get"}
// Dependencies: {}
# [doc = " Retrieve a pointer to a supported ciphersuite of the default process-wide crypto provider."] # [doc = ""] # [doc = " This function will return NULL if the `provider` is NULL, or if the index is out of bounds"] # [doc = " with respect to `rustls_default_crypto_provider_ciphersuites_len`."] # [doc = ""] # [doc = " The lifetime of the returned `rustls_supported_ciphersuite` is static, as the process-wide"] # [doc = " default provider lives for as long as the process."] # [no_mangle] pub extern "C" fn rustls_default_crypto_provider_ciphersuites_get (index : usize ,) -> * const rustls_supported_ciphersuite { ffi_panic_boundary ! { let default_provider = match get_default_or_install_from_crate_features () { Some (provider) => provider , None => return core :: ptr :: null () , } ; match default_provider . cipher_suites . get (index) { Some (ciphersuite) => ciphersuite as * const SupportedCipherSuite as * const _ , None => core :: ptr :: null () , } } }
};
}
