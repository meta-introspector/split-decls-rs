// Generated macro for KeyPkgVersion (enum)
macro_rules! Depcrate_symmetric_keyKeyPkgVersion {
() => {
// Module: crate::symmetric_key
// Provides: {"KeyPkgVersion"}
// Dependencies: {}
# [doc = " The `KeyPkgVersion` type is defined in [RFC 6031 Section 2.0]."] # [doc = ""] # [doc = " ```text"] # [doc = "     KeyPkgVersion ::= INTEGER  { v1(1) } ( v1, ... )"] # [doc = " ```"] # [doc = " [RFC 6031 Section 2.0]: https://datatracker.ietf.org/doc/html/rfc6031#section-2"] # [derive (Default , Clone , Debug , Copy , PartialEq , Eq , PartialOrd , Ord , Enumerated)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [allow (missing_docs)] pub enum KeyPkgVersion { # [default] V1 = 1 , }
};
}
