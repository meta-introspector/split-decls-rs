// Generated macro for impl_666 (impl)
macro_rules! Depcrate_stable_hasherimpl_666 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_666"}
// Dependencies: {}
impl < T1 : HashStable < CTX > , T2 : HashStable < CTX > , CTX > HashStable < CTX > for (T1 , T2) { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; } }
};
}
