// Generated macro for impl_683 (impl)
macro_rules! Depcrate_stable_hasherimpl_683 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_683"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for String { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (hcx , hasher) ; } }
};
}
