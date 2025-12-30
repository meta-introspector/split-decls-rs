// Generated macro for rustls_crypto_provider_ciphersuites_get (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_ciphersuites_get {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_ciphersuites_get"}
// Dependencies: {}
# [doc = " Retrieve a pointer to a supported ciphersuite of the `rustls_crypto_provider`."] # [doc = ""] # [doc = " This function will return NULL if the `provider` is NULL, or if the index is out of bounds"] # [doc = " with respect to `rustls_crypto_provider_ciphersuites_len`."] # [doc = ""] # [doc = " The lifetime of the returned `rustls_supported_ciphersuite` is equal to the lifetime of the"] # [doc = " `provider` and should not be used after the `provider` is freed."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_ciphersuites_get (provider : * const rustls_crypto_provider , index : usize ,) -> * const rustls_supported_ciphersuite { ffi_panic_boundary ! { match try_clone_arc ! (provider) . cipher_suites . get (index) { Some (ciphersuite) => ciphersuite as * const SupportedCipherSuite as * const _ , None => core :: ptr :: null () , } } }
};
}
