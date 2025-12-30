// Generated macro for PkiMessage (struct)
macro_rules! Depcrate_messagePkiMessage {
() => {
// Module: crate::message
// Provides: {"PkiMessage"}
// Dependencies: {}
# [doc = " The `PKIMessage` type is defined in [RFC 4210 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " PKIMessage ::= SEQUENCE {"] # [doc = "     header           PKIHeader,"] # [doc = "     body             PKIBody,"] # [doc = "     protection   [0] PKIProtection OPTIONAL,"] # [doc = "     extraCerts   [1] SEQUENCE SIZE (1..MAX) OF CMPCertificate"] # [doc = "     OPTIONAL }"] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1]: https://datatracker.ietf.org/doc/html/rfc4210#section-5.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PkiMessage < 'a > { pub header : PkiHeader < 'a > , pub body : PkiBody < 'a > , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "false" , optional = "true")] pub protection : Option < PkiProtection > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub extra_certs : Option < Vec < CmpCertificate > > , }
};
}
