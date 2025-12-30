// Generated macro for CrlAnnContent (type)
macro_rules! Depcrate_annCrlAnnContent {
() => {
// Module: crate::ann
// Provides: {"CrlAnnContent"}
// Dependencies: {}
# [doc = " The `CRLAnnContent` announcement is defined in [RFC 4210 Section 5.3.16]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CRLAnnContent ::= SEQUENCE OF CertificateList"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.16]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.16"] pub type CrlAnnContent = Vec < CertificateList > ;
};
}
