// Generated macro for CrlBag (struct)
macro_rules! Depcrate_crl_typeCrlBag {
() => {
// Module: crate::crl_type
// Provides: {"CrlBag"}
// Dependencies: {}
# [doc = " The `CertBag` type is defined in [RFC 7292 Section 4.2.4]."] # [doc = ""] # [doc = "```text"] # [doc = "     CRLBag ::= SEQUENCE {"] # [doc = "      crlId     BAG-TYPE.&id ({CRLTypes}),"] # [doc = "      crltValue [0] EXPLICIT BAG-TYPE.&Type ({CRLTypes}{@crlId})"] # [doc = "  }"] # [doc = "```"] # [doc = ""] # [doc = " [RFC 7292 Section 4.2.4]: https://www.rfc-editor.org/rfc/rfc7292#section-4.2.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CrlBag { pub crl_id : ObjectIdentifier , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT")] pub crl_value : CrlTypes , }
};
}
