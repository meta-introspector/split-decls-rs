// Generated macro for CertRequest (struct)
macro_rules! Depcrate_requestCertRequest {
() => {
// Module: crate::request
// Provides: {"CertRequest"}
// Dependencies: {}
# [doc = " The `CertRequest` type is defined in [RFC 4211 Section 5]."] # [doc = ""] # [doc = " ```text"] # [doc = "   CertRequest ::= SEQUENCE {"] # [doc = "       certReqId     INTEGER,"] # [doc = "       -- ID for matching request and reply"] # [doc = "       certTemplate  CertTemplate,"] # [doc = "       -- Selected fields of cert to be issued"] # [doc = "       controls      Controls OPTIONAL }"] # [doc = "       -- Attributes affecting issuance"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 5]: https://www.rfc-editor.org/rfc/rfc4211#section-5"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertRequest { pub cert_req_id : Int , pub cert_template : CertTemplate , pub controls : Option < Controls > , }
};
}
