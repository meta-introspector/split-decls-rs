// Generated macro for PkiPublicationInfo (struct)
macro_rules! Depcrate_controlsPkiPublicationInfo {
() => {
// Module: crate::controls
// Provides: {"PkiPublicationInfo"}
// Dependencies: {}
# [doc = " The `PKIPublicationInfo` control is defined in [RFC 4211 Section 6.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   PKIPublicationInfo ::= SEQUENCE {"] # [doc = "       action     INTEGER {"] # [doc = "                      dontPublish (0),"] # [doc = "                      pleasePublish (1) },"] # [doc = "       pubInfos  SEQUENCE SIZE (1..MAX) OF SinglePubInfo OPTIONAL }"] # [doc = "       -- pubInfos MUST NOT be present if action is \"dontPublish\""] # [doc = "       -- (if action is \"pleasePublish\" and pubInfos is omitted,"] # [doc = "       -- \"dontCare\" is assumed)"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 6.3]: https://www.rfc-editor.org/rfc/rfc4211#section-6.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PkiPublicationInfo { pub action : PkiPublicationInfoAction , pub pub_infos : Option < Vec < SinglePubInfo > > , }
};
}
