// Generated macro for CmsVersion (enum)
macro_rules! Depcrate_content_infoCmsVersion {
() => {
// Module: crate::content_info
// Provides: {"CmsVersion"}
// Dependencies: {}
# [doc = " The `OtherCertificateFormat` type is defined in [RFC 5652 Section 10.2.5]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CMSVersion ::= INTEGER  { v0(0), v1(1), v2(2), v3(3), v4(4), v5(5) }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.5]: https://www.rfc-editor.org/rfc/rfc5652#section-10.2.5"] # [derive (Clone , Debug , Copy , PartialEq , Eq , PartialOrd , Ord , Enumerated)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [allow (missing_docs)] pub enum CmsVersion { V0 = 0 , V1 = 1 , V2 = 2 , V3 = 3 , V4 = 4 , V5 = 5 , }
};
}
