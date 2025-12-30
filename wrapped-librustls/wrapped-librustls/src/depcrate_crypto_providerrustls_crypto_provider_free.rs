// Generated macro for rustls_crypto_provider_free (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_free {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_free"}
// Dependencies: {}
# [doc = " Frees the `rustls_crypto_provider`."] # [doc = ""] # [doc = " Calling with `NULL` is fine."] # [doc = " Must not be called twice with the same value."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_free (provider : * const rustls_crypto_provider) { ffi_panic_boundary ! { free_arc (provider) ; } }
};
}
