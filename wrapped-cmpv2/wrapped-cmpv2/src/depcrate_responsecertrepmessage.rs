// Generated macro for CertRepMessage (struct)
macro_rules! Depcrate_responseCertRepMessage {
() => {
// Module: crate::response
// Provides: {"CertRepMessage"}
// Dependencies: {}
# [doc = " The `CertRepMessage` type is defined in [RFC 4210 Section 5.3.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CertRepMessage ::= SEQUENCE {"] # [doc = "      caPubs       [1] SEQUENCE SIZE (1..MAX) OF CMPCertificate"] # [doc = "                    OPTIONAL,"] # [doc = "      response         SEQUENCE OF CertResponse }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.4]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertRepMessage < 'a > { # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub ca_pubs : Option < Vec < CmpCertificate > > , pub response : Vec < CertResponse < 'a > > , }
};
}
