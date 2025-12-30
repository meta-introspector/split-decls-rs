// Generated macro for impl_665 (impl)
macro_rules! Depcrate_stable_hasherimpl_665 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_665"}
// Dependencies: {}
impl < T1 : HashStable < CTX > , CTX > HashStable < CTX > for (T1 ,) { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 ,) = * self ; _0 . hash_stable (ctx , hasher) ; } }
};
}
