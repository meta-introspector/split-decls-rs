// Generated macro for macro_1622 (macro)
macro_rules! Depcrate_x509macro_1622 {
() => {
// Module: crate::x509
// Provides: {"macro_1622"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { use ffi :: X509_OBJECT_free ; } else { # [allow (bad_style)] unsafe fn X509_OBJECT_free (x : * mut ffi :: X509_OBJECT) { ffi :: X509_OBJECT_free_contents (x) ; ffi :: CRYPTO_free (x as * mut libc :: c_void) ; } } }
};
}
