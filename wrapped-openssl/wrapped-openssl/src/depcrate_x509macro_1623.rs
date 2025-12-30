// Generated macro for macro_1623 (macro)
macro_rules! Depcrate_x509macro_1623 {
() => {
// Module: crate::x509
// Provides: {"macro_1623"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { use ffi :: { X509_CRL_get_issuer , X509_CRL_get0_nextUpdate , X509_CRL_get0_lastUpdate , X509_CRL_get_REVOKED , X509_REVOKED_get0_revocationDate , X509_REVOKED_get0_serialNumber , } ; } else { # [allow (bad_style)] unsafe fn X509_CRL_get0_lastUpdate (x : * const ffi :: X509_CRL) -> * mut ffi :: ASN1_TIME { (* (* x) . crl) . lastUpdate } # [allow (bad_style)] unsafe fn X509_CRL_get0_nextUpdate (x : * const ffi :: X509_CRL) -> * mut ffi :: ASN1_TIME { (* (* x) . crl) . nextUpdate } # [allow (bad_style)] unsafe fn X509_CRL_get_issuer (x : * const ffi :: X509_CRL) -> * mut ffi :: X509_NAME { (* (* x) . crl) . issuer } # [allow (bad_style)] unsafe fn X509_CRL_get_REVOKED (x : * const ffi :: X509_CRL) -> * mut ffi :: stack_st_X509_REVOKED { (* (* x) . crl) . revoked } # [allow (bad_style)] unsafe fn X509_REVOKED_get0_serialNumber (x : * const ffi :: X509_REVOKED) -> * mut ffi :: ASN1_INTEGER { (* x) . serialNumber } # [allow (bad_style)] unsafe fn X509_REVOKED_get0_revocationDate (x : * const ffi :: X509_REVOKED) -> * mut ffi :: ASN1_TIME { (* x) . revocationDate } } }
};
}
