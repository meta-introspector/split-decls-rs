// Generated macro for impl_681 (impl)
macro_rules! Depcrate_stable_hasherimpl_681 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_681"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for str { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . as_bytes () . hash_stable (ctx , hasher) ; } }
};
}
