// Generated macro for macro_1448 (macro)
macro_rules! Depcrate_x509_storemacro_1448 {
() => {
// Module: crate::x509::store
// Provides: {"macro_1448"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: X509_STORE_get0_objects ; } else { # [allow (bad_style)] unsafe fn X509_STORE_get0_objects (x : * mut ffi :: X509_STORE) -> * mut ffi :: stack_st_X509_OBJECT { (* x) . objs } } }
};
}
