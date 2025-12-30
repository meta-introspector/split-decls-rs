// Generated macro for CertResponse (struct)
macro_rules! Depcrate_responseCertResponse {
() => {
// Module: crate::response
// Provides: {"CertResponse"}
// Dependencies: {}
# [doc = " The `CertResponse` type is defined in [RFC 4210 Section 5.3.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CertResponse ::= SEQUENCE {"] # [doc = "      certReqId           INTEGER,"] # [doc = "      -- to match this response with the corresponding request (a value"] # [doc = "      -- of -1 is to be used if certReqId is not specified in the"] # [doc = "      -- corresponding request)"] # [doc = "      status              PKIStatusInfo,"] # [doc = "      certifiedKeyPair    CertifiedKeyPair    OPTIONAL,"] # [doc = "      rspInfo             OCTET STRING        OPTIONAL"] # [doc = "      -- analogous to the id-regInfo-utf8Pairs string defined"] # [doc = "      -- for regInfo in CertReqMsg [RFC4211]"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.4]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertResponse < 'a > { pub cert_req_id : Int , pub status : PkiStatusInfo < 'a > , pub certified_key_pair : Option < CertifiedKeyPair > , pub rsp_info : Option < OctetString > , }
};
}
