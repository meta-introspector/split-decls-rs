// Generated macro for macro_105 (macro)
macro_rules! Depcrate_asn1macro_105 {
() => {
// Module: crate::asn1
// Provides: {"macro_105"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: ASN1_INTEGER ; fn drop = ffi :: ASN1_INTEGER_free ; # [doc = " Numeric representation"] # [doc = ""] # [doc = " Integers in ASN.1 may include BigNum, int64 or uint64.  BigNum implementation"] # [doc = " can be found within [`bn`] module."] # [doc = ""] # [doc = " OpenSSL documentation includes [`ASN1_INTEGER_set`]."] # [doc = ""] # [doc = " [`bn`]: ../bn/index.html"] # [doc = " [`ASN1_INTEGER_set`]: https://docs.openssl.org/master/man3/ASN1_INTEGER_set/"] pub struct Asn1Integer ; # [doc = " A reference to an [`Asn1Integer`]."] pub struct Asn1IntegerRef ; }
};
}
