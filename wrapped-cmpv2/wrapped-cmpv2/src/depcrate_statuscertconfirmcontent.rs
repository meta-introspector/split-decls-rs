// Generated macro for CertConfirmContent (type)
macro_rules! Depcrate_statusCertConfirmContent {
() => {
// Module: crate::status
// Provides: {"CertConfirmContent"}
// Dependencies: {}
# [doc = " The `CertConfirmContent` type is defined in [RFC 4210 Section 5.2.18]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CertConfirmContent ::= SEQUENCE OF CertStatus"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.18]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.18"] pub type CertConfirmContent < 'a > = Vec < CertStatus < 'a > > ;
};
}
