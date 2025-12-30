// Generated macro for impl_677 (impl)
macro_rules! Depcrate_stable_hasherimpl_677 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_677"}
// Dependencies: {}
impl < A , const N : usize , CTX > HashStable < CTX > for SmallVec < [A ; N] > where A : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }
};
}
