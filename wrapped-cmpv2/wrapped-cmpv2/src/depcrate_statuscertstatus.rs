// Generated macro for CertStatus (struct)
macro_rules! Depcrate_statusCertStatus {
() => {
// Module: crate::status
// Provides: {"CertStatus"}
// Dependencies: {}
# [doc = " The `CertStatus` type is defined in [RFC 4210 Section 5.2.18]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CertStatus ::= SEQUENCE {"] # [doc = "      certHash    OCTET STRING,"] # [doc = "      -- the hash of the certificate, using the same hash algorithm"] # [doc = "      -- as is used to create and verify the certificate signature"] # [doc = "      certReqId   INTEGER,"] # [doc = "      -- to match this confirmation with the corresponding req/rep"] # [doc = "      statusInfo  PKIStatusInfo OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.18]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.18"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertStatus < 'a > { pub cert_hash : OctetString , pub cert_req_id : Int , pub status_info : Option < PkiStatusInfo < 'a > > , }
};
}
