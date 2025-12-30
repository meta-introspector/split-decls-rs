// Generated macro for impl_674 (impl)
macro_rules! Depcrate_stable_hasherimpl_674 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_674"}
// Dependencies: {}
impl < T : HashStable < CTX > , CTX > HashStable < CTX > for Vec < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }
};
}
