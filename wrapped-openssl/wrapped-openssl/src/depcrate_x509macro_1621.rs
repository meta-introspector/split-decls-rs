// Generated macro for macro_1621 (macro)
macro_rules! Depcrate_x509macro_1621 {
() => {
// Module: crate::x509
// Provides: {"macro_1621"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , boringssl , libressl , awslc))] { use ffi :: X509_OBJECT_get0_X509 ; } else { # [allow (bad_style)] unsafe fn X509_OBJECT_get0_X509 (x : * mut ffi :: X509_OBJECT) -> * mut ffi :: X509 { if (* x) . type_ == ffi :: X509_LU_X509 { (* x) . data . x509 } else { ptr :: null_mut () } } } }
};
}
