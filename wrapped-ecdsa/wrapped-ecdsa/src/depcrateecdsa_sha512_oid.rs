// Generated macro for ECDSA_SHA512_OID (const)
macro_rules! DepcrateECDSA_SHA512_OID {
() => {
// Module: crate
// Provides: {"ECDSA_SHA512_OID"}
// Dependencies: {}
# [doc = " OID for ECDSA with SHA-512 digests."] # [doc = ""] # [doc = " ```text"] # [doc = " ecdsa-with-SHA512 OBJECT IDENTIFIER ::= { iso(1) member-body(2)"] # [doc = "      us(840) ansi-X9-62(10045) signatures(4) ecdsa-with-SHA2(3) 4 }"] # [doc = " ```"] # [cfg (feature = "digest")] pub const ECDSA_SHA512_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("1.2.840.10045.4.3.4") ;
};
}
