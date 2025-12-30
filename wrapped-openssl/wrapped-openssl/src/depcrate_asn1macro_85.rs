// Generated macro for macro_85 (macro)
macro_rules! Depcrate_asn1macro_85 {
() => {
// Module: crate::asn1
// Provides: {"macro_85"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: ASN1_TIME ; fn drop = ffi :: ASN1_TIME_free ; # [doc = " Time storage and comparison"] # [doc = ""] # [doc = " Asn1Time should be used to store and share time information"] # [doc = " using certificates.  If Asn1Time is set using a string, it must"] # [doc = " be in either YYMMDDHHMMSSZ, YYYYMMDDHHMMSSZ, or another ASN.1 format."] # [doc = ""] # [doc = " [ASN_TIME_set] documentation at OpenSSL explains the ASN.1 implementation"] # [doc = " used by OpenSSL."] # [doc = ""] # [doc = " [ASN_TIME_set]: https://docs.openssl.org/master/man3/ASN1_TIME_set/"] pub struct Asn1Time ; # [doc = " Reference to an [`Asn1Time`]"] # [doc = ""] # [doc = " [`Asn1Time`]: struct.Asn1Time.html"] pub struct Asn1TimeRef ; }
};
}
