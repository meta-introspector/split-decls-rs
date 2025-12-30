// Generated macro for ProtectedPart (struct)
macro_rules! Depcrate_messageProtectedPart {
() => {
// Module: crate::message
// Provides: {"ProtectedPart"}
// Dependencies: {}
# [doc = " The `ProtectedPart` type is defined in [RFC 4210 Section 5.1.3]."] # [doc = ""] # [doc = " ```text"] # [doc = " ProtectedPart ::= SEQUENCE {"] # [doc = "     header    PKIHeader,"] # [doc = "     body      PKIBody }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1.3]: https://www.rfc-editor.org/rfc/rfc4210#section-5.1.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ProtectedPart < 'a > { pub header : PkiHeader < 'a > , pub body : PkiBody < 'a > , }
};
}
