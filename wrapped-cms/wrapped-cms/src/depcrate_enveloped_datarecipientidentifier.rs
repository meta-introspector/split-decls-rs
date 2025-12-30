// Generated macro for RecipientIdentifier (enum)
macro_rules! Depcrate_enveloped_dataRecipientIdentifier {
() => {
// Module: crate::enveloped_data
// Provides: {"RecipientIdentifier"}
// Dependencies: {}
# [doc = " The `RecipientIdentifier` type is defined in [RFC 5652 Section 6.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RecipientIdentifier ::= CHOICE {"] # [doc = "       issuerAndSerialNumber IssuerAndSerialNumber,"] # [doc = "       ...,"] # [doc = "       [[2: subjectKeyIdentifier [0] SubjectKeyIdentifier ]] }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.1]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum RecipientIdentifier { IssuerAndSerialNumber (IssuerAndSerialNumber) , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT")] SubjectKeyIdentifier (SubjectKeyIdentifier) , }
};
}
