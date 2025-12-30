// Generated macro for RevAnnContent (struct)
macro_rules! Depcrate_annRevAnnContent {
() => {
// Module: crate::ann
// Provides: {"RevAnnContent"}
// Dependencies: {}
# [doc = " The `RevAnnContent` announcement is defined in [RFC 4210 Section 5.3.15]."] # [doc = ""] # [doc = " ```text"] # [doc = "  RevAnnContent ::= SEQUENCE {"] # [doc = "      status              PKIStatus,"] # [doc = "      certId              CertId,"] # [doc = "      willBeRevokedAt     GeneralizedTime,"] # [doc = "      badSinceDate        GeneralizedTime,"] # [doc = "      crlDetails          Extensions{{...}}  OPTIONAL"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.15]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.15"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct RevAnnContent { pub status : PkiStatus , pub cert_id : CertId , pub will_be_revoked_at : GeneralizedTime , pub bad_since_date : GeneralizedTime , pub crl_details : Option < Extensions > , }
};
}
