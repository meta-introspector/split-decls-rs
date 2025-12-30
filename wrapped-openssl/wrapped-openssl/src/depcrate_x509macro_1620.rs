// Generated macro for macro_1620 (macro)
macro_rules! Depcrate_x509macro_1620 {
() => {
// Module: crate::x509
// Provides: {"macro_1620"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: { X509_ALGOR_get0 , ASN1_STRING_get0_data , X509_STORE_CTX_get0_chain , X509_set1_notAfter , X509_set1_notBefore , X509_REQ_get_version , X509_REQ_get_subject_name , } ; } else { use ffi :: { ASN1_STRING_data as ASN1_STRING_get0_data , X509_STORE_CTX_get_chain as X509_STORE_CTX_get0_chain , X509_set_notAfter as X509_set1_notAfter , X509_set_notBefore as X509_set1_notBefore , } ; # [allow (bad_style)] unsafe fn X509_REQ_get_version (x : * mut ffi :: X509_REQ) -> :: libc :: c_long { ffi :: ASN1_INTEGER_get ((* (* x) . req_info) . version) } # [allow (bad_style)] unsafe fn X509_REQ_get_subject_name (x : * mut ffi :: X509_REQ) -> * mut :: ffi :: X509_NAME { (* (* x) . req_info) . subject } # [allow (bad_style)] unsafe fn X509_ALGOR_get0 (paobj : * mut * const ffi :: ASN1_OBJECT , pptype : * mut c_int , pval : * mut * mut :: libc :: c_void , alg : * const ffi :: X509_ALGOR ,) { if ! paobj . is_null () { * paobj = (* alg) . algorithm ; } assert ! (pptype . is_null ()) ; assert ! (pval . is_null ()) ; } } }
};
}
