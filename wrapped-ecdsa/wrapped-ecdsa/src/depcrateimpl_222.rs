// Generated macro for impl_222 (impl)
macro_rules! Depcrateimpl_222 {
() => {
// Module: crate
// Provides: {"impl_222"}
// Dependencies: {}
# [doc = " NOTE: this implementation assumes the default digest for the given elliptic"] # [doc = " curve as defined by [`hazmat::DigestAlgorithm`]."] # [doc = ""] # [doc = " When working with alternative digests, you will need to use e.g."] # [doc = " [`SignatureWithOid::new_with_digest`]."] # [cfg (all (feature = "digest" , feature = "hazmat"))] impl < C > SignatureEncoding for SignatureWithOid < C > where C : hazmat :: DigestAlgorithm , C :: Digest : AssociatedOid , SignatureSize < C > : ArraySize , { type Repr = SignatureBytes < C > ; }
};
}
