// Generated macro for RecipientKeyIdentifier (struct)
macro_rules! Depcrate_enveloped_dataRecipientKeyIdentifier {
() => {
// Module: crate::enveloped_data
// Provides: {"RecipientKeyIdentifier"}
// Dependencies: {}
# [doc = " The `RecipientKeyIdentifier` type is defined in [RFC 5652 Section 6.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RecipientKeyIdentifier ::= SEQUENCE {"] # [doc = "       subjectKeyIdentifier SubjectKeyIdentifier,"] # [doc = "       date GeneralizedTime OPTIONAL,"] # [doc = "       other OtherKeyAttribute OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct RecipientKeyIdentifier { pub subject_key_identifier : SubjectKeyIdentifier , pub date : Option < GeneralizedTime > , pub other : Option < Attribute > , }
};
}
