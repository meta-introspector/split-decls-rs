// Generated macro for KekIdentifier (struct)
macro_rules! Depcrate_enveloped_dataKekIdentifier {
() => {
// Module: crate::enveloped_data
// Provides: {"KekIdentifier"}
// Dependencies: {}
# [doc = " The `KEKIdentifier` type is defined in [RFC 5652 Section 6.2.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   KEKIdentifier ::= SEQUENCE {"] # [doc = "       keyIdentifier OCTET STRING,"] # [doc = "       date GeneralizedTime OPTIONAL,"] # [doc = "       other OtherKeyAttribute OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.3]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct KekIdentifier { pub kek_identifier : OctetString , pub date : Option < GeneralizedTime > , pub other : Option < Attribute > , }
};
}
