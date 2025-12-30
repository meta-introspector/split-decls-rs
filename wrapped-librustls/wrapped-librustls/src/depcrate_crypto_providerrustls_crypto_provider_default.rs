// Generated macro for rustls_crypto_provider_default (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_default {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_default"}
// Dependencies: {}
# [doc = " Retrieve a pointer to the process default `rustls_crypto_provider`."] # [doc = ""] # [doc = " This may return `NULL` if no process default provider has been set using"] # [doc = " `rustls_crypto_provider_builder_build_default`."] # [doc = ""] # [doc = " Caller owns the returned `rustls_crypto_provider` and must free it w/ `rustls_crypto_provider_free`."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_default () -> * const rustls_crypto_provider { ffi_panic_boundary ! { match CryptoProvider :: get_default () { Some (provider) => Arc :: into_raw (provider . clone ()) as * const rustls_crypto_provider , None => core :: ptr :: null () , } } }
};
}
