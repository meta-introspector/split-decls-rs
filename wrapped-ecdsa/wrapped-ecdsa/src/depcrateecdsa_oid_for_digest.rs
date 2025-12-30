// Generated macro for ecdsa_oid_for_digest (function)
macro_rules! Depcrateecdsa_oid_for_digest {
() => {
// Module: crate
// Provides: {"ecdsa_oid_for_digest"}
// Dependencies: {}
# [doc = " Get the ECDSA OID for a given digest OID."] # [cfg (feature = "digest")] const fn ecdsa_oid_for_digest (digest_oid : ObjectIdentifier) -> Option < ObjectIdentifier > { match digest_oid { SHA224_OID => Some (ECDSA_SHA224_OID) , SHA256_OID => Some (ECDSA_SHA256_OID) , SHA384_OID => Some (ECDSA_SHA384_OID) , SHA512_OID => Some (ECDSA_SHA512_OID) , _ => None , } }
};
}
