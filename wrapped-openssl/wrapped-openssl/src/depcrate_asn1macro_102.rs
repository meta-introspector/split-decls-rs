// Generated macro for macro_102 (macro)
macro_rules! Depcrate_asn1macro_102 {
() => {
// Module: crate::asn1
// Provides: {"macro_102"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: ASN1_STRING ; fn drop = ffi :: ASN1_STRING_free ; # [doc = " Primary ASN.1 type used by OpenSSL"] # [doc = ""] # [doc = " Almost all ASN.1 types in OpenSSL are represented by ASN1_STRING"] # [doc = " structures.  This implementation uses [ASN1_STRING-to_UTF8] to preserve"] # [doc = " compatibility with Rust's String."] # [doc = ""] # [doc = " [ASN1_STRING-to_UTF8]: https://docs.openssl.org/master/man3/ASN1_STRING_to_UTF8/"] pub struct Asn1String ; # [doc = " A reference to an [`Asn1String`]."] pub struct Asn1StringRef ; }
};
}
