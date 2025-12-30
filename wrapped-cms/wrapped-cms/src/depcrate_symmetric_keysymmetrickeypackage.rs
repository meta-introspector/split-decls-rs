// Generated macro for SymmetricKeyPackage (struct)
macro_rules! Depcrate_symmetric_keySymmetricKeyPackage {
() => {
// Module: crate::symmetric_key
// Provides: {"SymmetricKeyPackage"}
// Dependencies: {}
# [doc = " The `SymmetricKeyPackage` type is defined in [RFC 6031 Section 2.0]."] # [doc = ""] # [doc = " ```text"] # [doc = "      SymmetricKeyPackage ::= SEQUENCE {"] # [doc = "        version           KeyPkgVersion DEFAULT v1,"] # [doc = "        sKeyPkgAttrs  [0] SEQUENCE SIZE (1..MAX) OF Attribute"] # [doc = "                                       {{ SKeyPkgAttributes }} OPTIONAL,"] # [doc = "        sKeys             SymmetricKeys,"] # [doc = "        ... }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 2.0]: https://datatracker.ietf.org/doc/html/rfc6031#section-2"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct SymmetricKeyPackage { # [asn1 (default = "Default::default")] pub version : KeyPkgVersion , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub s_key_pkg_attrs : Option < Vec < Attribute > > , pub s_keys : SymmetricKeys , }
};
}
