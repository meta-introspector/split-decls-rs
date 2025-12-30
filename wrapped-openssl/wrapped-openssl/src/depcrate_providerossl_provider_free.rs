// Generated macro for ossl_provider_free (function)
macro_rules! Depcrate_providerossl_provider_free {
() => {
// Module: crate::provider
// Provides: {"ossl_provider_free"}
// Dependencies: {}
# [inline] unsafe fn ossl_provider_free (p : * mut ffi :: OSSL_PROVIDER) { ffi :: OSSL_PROVIDER_unload (p) ; }
};
}
