// Generated macro for rustls_crypto_provider_builder_build_as_default (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_builder_build_as_default {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_builder_build_as_default"}
// Dependencies: {}
# [doc = " Builds a `rustls_crypto_provider` from the builder and sets it as the"] # [doc = " process-wide default crypto provider."] # [doc = ""] # [doc = " Afterward, the default provider can be retrieved using `rustls_crypto_provider_default`."] # [doc = ""] # [doc = " This can only be done once per process, and will return an error if a"] # [doc = " default provider has already been set, or if the builder has already been built."] # [doc = ""] # [doc = " The `rustls_crypto_provider_builder` builder is consumed and should not be used"] # [doc = " for further calls, except to `rustls_crypto_provider_builder_free`. The caller must"] # [doc = " still free the builder after a successful build."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_builder_build_as_default (builder : * mut rustls_crypto_provider_builder ,) -> rustls_result { let builder = try_mut_from_ptr ! (builder) ; match try_take ! (builder) . build_provider () . install_default () { Ok (_) => rustls_result :: Ok , Err (_) => rustls_result :: AlreadyUsed , } }
};
}
