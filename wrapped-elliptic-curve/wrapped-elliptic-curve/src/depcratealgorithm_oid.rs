// Generated macro for ALGORITHM_OID (const)
macro_rules! DepcrateALGORITHM_OID {
() => {
// Module: crate
// Provides: {"ALGORITHM_OID"}
// Dependencies: {}
# [doc = " Algorithm [`ObjectIdentifier`][`pkcs8::ObjectIdentifier`] for elliptic curve public key"] # [doc = " cryptography (`id-ecPublicKey`)."] # [doc = ""] # [doc = " <https://oid-base.com/get/1.2.840.10045.2.1>"] # [cfg (feature = "pkcs8")] pub const ALGORITHM_OID : pkcs8 :: ObjectIdentifier = pkcs8 :: ObjectIdentifier :: new_unwrap ("1.2.840.10045.2.1") ;
};
}
