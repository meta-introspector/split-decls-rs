// Generated macro for macro_1449 (macro)
macro_rules! Depcrate_x509_storemacro_1449 {
() => {
// Module: crate::x509::store
// Provides: {"macro_1449"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl110)] { use ffi :: X509_LOOKUP_meth_free ; } else { # [allow (bad_style)] unsafe fn X509_LOOKUP_meth_free (_x : * mut ffi :: X509_LOOKUP_METHOD) { } } }
};
}
