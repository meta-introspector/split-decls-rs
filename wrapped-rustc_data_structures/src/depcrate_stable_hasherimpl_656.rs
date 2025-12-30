// Generated macro for impl_656 (impl)
macro_rules! Depcrate_stable_hasherimpl_656 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_656"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for Hash128 { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { self . as_u128 () . hash (hasher) ; } }
};
}
