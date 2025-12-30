// Generated macro for CertReqMsg (struct)
macro_rules! Depcrate_requestCertReqMsg {
() => {
// Module: crate::request
// Provides: {"CertReqMsg"}
// Dependencies: {}
# [doc = " The `CertReqMsg` type is defined in [RFC 4211 Section 3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   CertReqMsg ::= SEQUENCE {"] # [doc = "       certReq   CertRequest,"] # [doc = "       popo       ProofOfPossession  OPTIONAL,"] # [doc = "       -- content depends upon key type"] # [doc = "       regInfo   SEQUENCE SIZE(1..MAX) OF"] # [doc = "           SingleAttribute{{RegInfoSet}} OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 3]: https://www.rfc-editor.org/rfc/rfc4211#section-3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertReqMsg { pub cert_req : CertRequest , pub popo : Option < ProofOfPossession > , pub reg_info : Option < AttributeSeq > , }
};
}
