// Generated macro for macro_1619 (macro)
macro_rules! Depcrate_x509macro_1619 {
() => {
// Module: crate::x509
// Provides: {"macro_1619"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: { X509_getm_notAfter , X509_getm_notBefore , X509_up_ref , X509_get0_signature } ; } else { # [allow (bad_style)] unsafe fn X509_getm_notAfter (x : * mut ffi :: X509) -> * mut ffi :: ASN1_TIME { (* (* (* x) . cert_info) . validity) . notAfter } # [allow (bad_style)] unsafe fn X509_getm_notBefore (x : * mut ffi :: X509) -> * mut ffi :: ASN1_TIME { (* (* (* x) . cert_info) . validity) . notBefore } # [allow (bad_style)] unsafe fn X509_up_ref (x : * mut ffi :: X509) { ffi :: CRYPTO_add_lock (& mut (* x) . references , 1 , ffi :: CRYPTO_LOCK_X509 , "mod.rs\0" . as_ptr () as * const _ , line ! () as c_int ,) ; } # [allow (bad_style)] unsafe fn X509_get0_signature (psig : * mut * const ffi :: ASN1_BIT_STRING , palg : * mut * const ffi :: X509_ALGOR , x : * const ffi :: X509 ,) { if ! psig . is_null () { * psig = (* x) . signature ; } if ! palg . is_null () { * palg = (* x) . sig_alg ; } } } }
};
}
