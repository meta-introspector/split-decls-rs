// Generated macro for impl_1625 (impl)
macro_rules! Depcrate_x509impl_1625 {
() => {
// Module: crate::x509
// Provides: {"impl_1625"}
// Dependencies: {}
impl X509PurposeId { pub const SSL_CLIENT : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_SSL_CLIENT) ; pub const SSL_SERVER : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_SSL_SERVER) ; pub const NS_SSL_SERVER : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_NS_SSL_SERVER) ; pub const SMIME_SIGN : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_SMIME_SIGN) ; pub const SMIME_ENCRYPT : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_SMIME_ENCRYPT) ; pub const CRL_SIGN : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_CRL_SIGN) ; pub const ANY : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_ANY) ; pub const OCSP_HELPER : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_OCSP_HELPER) ; pub const TIMESTAMP_SIGN : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_TIMESTAMP_SIGN) ; # [cfg (ossl320)] pub const CODE_SIGN : X509PurposeId = X509PurposeId (ffi :: X509_PURPOSE_CODE_SIGN) ; # [doc = " Constructs an `X509PurposeId` from a raw OpenSSL value."] pub fn from_raw (id : c_int) -> Self { X509PurposeId (id) } # [doc = " Returns the raw OpenSSL value represented by this type."] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
