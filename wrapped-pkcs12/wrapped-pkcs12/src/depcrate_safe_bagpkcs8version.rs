// Generated macro for Pkcs8Version (enum)
macro_rules! Depcrate_safe_bagPkcs8Version {
() => {
// Module: crate::safe_bag
// Provides: {"Pkcs8Version"}
// Dependencies: {}
# [doc = " Version for the PrivateKeyInfo structure as defined in [RFC 5208 Section 5]."] # [doc = ""] # [doc = " [RFC 5208 Section 5]: https://www.rfc-editor.org/rfc/rfc5208#section-5"] # [derive (Clone , Copy , Debug , Enumerated , Eq , PartialEq , PartialOrd , Ord)] # [asn1 (type = "INTEGER")] # [repr (u8)] pub enum Pkcs8Version { # [doc = " syntax version 3"] V0 = 0 , }
};
}
