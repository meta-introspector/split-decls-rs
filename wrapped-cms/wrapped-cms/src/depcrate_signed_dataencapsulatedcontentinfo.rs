// Generated macro for EncapsulatedContentInfo (struct)
macro_rules! Depcrate_signed_dataEncapsulatedContentInfo {
() => {
// Module: crate::signed_data
// Provides: {"EncapsulatedContentInfo"}
// Dependencies: {}
# [doc = " The `EncapsulatedContentInfo` type is defined in [RFC 5652 Section 5.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   EncapsulatedContentInfo ::= SEQUENCE {"] # [doc = "       eContentType       CONTENT-TYPE.&id({ContentSet}),"] # [doc = "       eContent           [0] EXPLICIT OCTET STRING"] # [doc = "               ( CONTAINING CONTENT-TYPE."] # [doc = "                   &Type({ContentSet}{@eContentType})) OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 5.2]: https://www.rfc-editor.org/rfc/rfc5652#section-5.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EncapsulatedContentInfo { pub econtent_type : ObjectIdentifier , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , optional = "true")] pub econtent : Option < Any > , }
};
}
