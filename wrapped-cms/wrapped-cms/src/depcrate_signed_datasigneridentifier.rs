// Generated macro for SignerIdentifier (enum)
macro_rules! Depcrate_signed_dataSignerIdentifier {
() => {
// Module: crate::signed_data
// Provides: {"SignerIdentifier"}
// Dependencies: {}
# [doc = " The `SignerIdentifier` type is defined in [RFC 5652 Section 5.3]."] # [doc = ""] # [doc = " ```text"] # [doc = " SignerIdentifier ::= CHOICE {"] # [doc = "   issuerAndSerialNumber IssuerAndSerialNumber,"] # [doc = "   subjectKeyIdentifier \\[0\\] SubjectKeyIdentifier }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 5.3]: https://datatracker.ietf.org/doc/html/rfc5652#section-5.3"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum SignerIdentifier { IssuerAndSerialNumber (IssuerAndSerialNumber) , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT")] SubjectKeyIdentifier (SubjectKeyIdentifier) , }
};
}
