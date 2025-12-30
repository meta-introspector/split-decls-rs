// Generated macro for OtherCertificateFormat (struct)
macro_rules! Depcrate_certOtherCertificateFormat {
() => {
// Module: crate::cert
// Provides: {"OtherCertificateFormat"}
// Dependencies: {}
# [doc = " The `OtherCertificateFormat` type is defined in [RFC 5652 Section 10.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OtherCertificateFormat ::= SEQUENCE {"] # [doc = "       otherCertFormat OTHER-CERT-FMT."] # [doc = "               &id({SupportedCertFormats}),"] # [doc = "       otherCert       OTHER-CERT-FMT."] # [doc = "               &Type({SupportedCertFormats}{@otherCertFormat})}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-10.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OtherCertificateFormat { pub other_cert_format : ObjectIdentifier , pub other_cert : Any , }
};
}
