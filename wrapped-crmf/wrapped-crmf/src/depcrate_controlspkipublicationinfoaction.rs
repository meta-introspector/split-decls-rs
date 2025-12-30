// Generated macro for PkiPublicationInfoAction (enum)
macro_rules! Depcrate_controlsPkiPublicationInfoAction {
() => {
// Module: crate::controls
// Provides: {"PkiPublicationInfoAction"}
// Dependencies: {}
# [doc = " The `PKIPublicationInfo` control is defined [RFC 4211 Section 6.3] features"] # [doc = " an inline INTEGER definition that is implemented as the PkiPublicationInfoAction enum."] # [doc = ""] # [doc = " [RFC 4211 Section 6.3]: https://www.rfc-editor.org/rfc/rfc4211#section-6.3"] # [derive (Clone , Debug , Copy , PartialEq , Eq , Enumerated , Ord , PartialOrd)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [allow (missing_docs)] pub enum PkiPublicationInfoAction { DontPublish = 0 , PleasePublish = 1 , }
};
}
