// Generated macro for ContentInfo (struct)
macro_rules! Depcrate_content_infoContentInfo {
() => {
// Module: crate::content_info
// Provides: {"ContentInfo"}
// Dependencies: {}
# [doc = " The `ContentInfo` type is defined in [RFC 5652 Section 3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   ContentInfo ::= SEQUENCE {"] # [doc = "       contentType        CONTENT-TYPE."] # [doc = "                       &id({ContentSet}),"] # [doc = "       content            [0] EXPLICIT CONTENT-TYPE."] # [doc = "                       &Type({ContentSet}{@contentType})}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 3]: https://www.rfc-editor.org/rfc/rfc5652#section-3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ContentInfo { pub content_type : ObjectIdentifier , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT")] pub content : Any , }
};
}
