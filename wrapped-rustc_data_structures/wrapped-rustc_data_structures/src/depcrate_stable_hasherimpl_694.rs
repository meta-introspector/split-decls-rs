// Generated macro for impl_694 (impl)
macro_rules! Depcrate_stable_hasherimpl_694 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_694"}
// Dependencies: {}
impl < T , CTX > HashStable < CTX > for :: std :: ops :: RangeInclusive < T > where T : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . start () . hash_stable (ctx , hasher) ; self . end () . hash_stable (ctx , hasher) ; } }
};
}
