// Generated macro for impl_210 (impl)
macro_rules! Depcrateimpl_210 {
() => {
// Module: crate
// Provides: {"impl_210"}
// Dependencies: {}
# [doc = " ECDSA [`ObjectIdentifier`] which identifies the digest used by default"] # [doc = " with the `Signer` and `Verifier` traits."] # [doc = ""] # [doc = " To support non-default digest algorithms, use the [`SignatureWithOid`]"] # [doc = " type instead."] # [cfg (all (feature = "digest" , feature = "hazmat"))] impl < C > AssociatedOid for Signature < C > where C : hazmat :: DigestAlgorithm , C :: Digest : AssociatedOid , { const OID : ObjectIdentifier = match ecdsa_oid_for_digest (C :: Digest :: OID) { Some (oid) => oid , None => panic ! ("no RFC5758 ECDSA OID defined for DigestAlgorithm::Digest") , } ; }
};
}
