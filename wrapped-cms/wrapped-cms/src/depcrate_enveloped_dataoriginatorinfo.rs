// Generated macro for OriginatorInfo (struct)
macro_rules! Depcrate_enveloped_dataOriginatorInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"OriginatorInfo"}
// Dependencies: {}
# [doc = " The `OriginatorInfo` type is defined in [RFC 5652 Section 6.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OriginatorInfo ::= SEQUENCE {"] # [doc = "       certs [0] IMPLICIT CertificateSet OPTIONAL,"] # [doc = "       crls [1] IMPLICIT RevocationInfoChoices OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.1]: https://www.rfc-editor.org/rfc/rfc5652#section-6.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OriginatorInfo { # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub certs : Option < CertificateSet > , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub crls : Option < RevocationInfoChoices > , }
};
}
