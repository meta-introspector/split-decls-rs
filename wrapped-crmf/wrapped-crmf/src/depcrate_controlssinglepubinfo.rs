// Generated macro for SinglePubInfo (struct)
macro_rules! Depcrate_controlsSinglePubInfo {
() => {
// Module: crate::controls
// Provides: {"SinglePubInfo"}
// Dependencies: {}
# [doc = " The `SinglePubInfo` control is defined in [RFC 4211 Section 6.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   SinglePubInfo ::= SEQUENCE {"] # [doc = "       pubMethod    INTEGER {"] # [doc = "           dontCare    (0),"] # [doc = "           x500        (1),"] # [doc = "           web         (2),"] # [doc = "           ldap        (3) },"] # [doc = "       pubLocation  GeneralName OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 6.3]: https://www.rfc-editor.org/rfc/rfc4211#section-6.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct SinglePubInfo { pub pub_method : SinglePubInfoMethod , pub pub_location : Option < GeneralName > , }
};
}
