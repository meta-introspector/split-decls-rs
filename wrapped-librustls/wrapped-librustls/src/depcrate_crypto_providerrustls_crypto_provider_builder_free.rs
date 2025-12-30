// Generated macro for rustls_crypto_provider_builder_free (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_builder_free {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_builder_free"}
// Dependencies: {}
# [doc = " Free the `rustls_crypto_provider_builder`."] # [doc = ""] # [doc = " Calling with `NULL` is fine."] # [doc = " Must not be called twice with the same value."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_builder_free (builder : * mut rustls_crypto_provider_builder ,) { ffi_panic_boundary ! { free_box (builder) ; } }
};
}
