// Generated macro for CertBag (struct)
macro_rules! Depcrate_cert_typeCertBag {
() => {
// Module: crate::cert_type
// Provides: {"CertBag"}
// Dependencies: {}
# [doc = " The `CertBag` type is defined in [RFC 7292 Section 4.2.3]."] # [doc = ""] # [doc = "```text"] # [doc = " CertBag ::= SEQUENCE {"] # [doc = "     certId      BAG-TYPE.&id   ({CertTypes}),"] # [doc = "     certValue   [0] EXPLICIT BAG-TYPE.&Type ({CertTypes}{@certId})"] # [doc = " }"] # [doc = "```"] # [doc = ""] # [doc = " [RFC 7292 Section 4.2.3]: https://www.rfc-editor.org/rfc/rfc7292#section-4.2.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertBag { pub cert_id : ObjectIdentifier , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT")] pub cert_value : CertTypes , }
};
}
