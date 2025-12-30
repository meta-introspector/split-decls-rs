// Generated macro for SinglePubInfoMethod (enum)
macro_rules! Depcrate_controlsSinglePubInfoMethod {
() => {
// Module: crate::controls
// Provides: {"SinglePubInfoMethod"}
// Dependencies: {}
# [doc = " The `SinglePubInfo` control is defined [RFC 4211 Section 6.3] features"] # [doc = " an inline INTEGER definition that is implemented as the SinglePubInfoMethod enum."] # [doc = ""] # [doc = " [RFC 4211 Section 6.3]: https://www.rfc-editor.org/rfc/rfc4211#section-6.3"] # [derive (Clone , Debug , Copy , PartialEq , Eq , Enumerated , Ord , PartialOrd)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [allow (missing_docs)] pub enum SinglePubInfoMethod { DontCare = 0 , X500 = 1 , Web = 2 , Ldap = 3 , }
};
}
