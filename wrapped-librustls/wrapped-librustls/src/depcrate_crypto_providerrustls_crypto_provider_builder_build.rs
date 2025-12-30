// Generated macro for rustls_crypto_provider_builder_build (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_builder_build {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_builder_build"}
// Dependencies: {}
# [doc = " Builds a `rustls_crypto_provider` from the builder and returns it. Returns an error if the"] # [doc = " builder has already been built."] # [doc = ""] # [doc = " The `rustls_crypto_provider_builder` builder is consumed and should not be used"] # [doc = " for further calls, except to `rustls_crypto_provider_builder_free`. The caller must"] # [doc = " still free the builder after a successful build."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_builder_build (builder : * mut rustls_crypto_provider_builder , provider_out : * mut * const rustls_crypto_provider ,) -> rustls_result { ffi_panic_boundary ! { let builder = try_mut_from_ptr ! (builder) ; set_arc_mut_ptr (try_ref_from_ptr_ptr ! (provider_out) , try_take ! (builder) . build_provider () ,) ; rustls_result :: Ok } }
};
}
