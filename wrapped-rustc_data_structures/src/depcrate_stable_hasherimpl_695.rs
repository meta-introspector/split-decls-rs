// Generated macro for impl_695 (impl)
macro_rules! Depcrate_stable_hasherimpl_695 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_695"}
// Dependencies: {}
impl < I : Idx , T , CTX > HashStable < CTX > for IndexSlice < I , T > where T : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for v in & self . raw { v . hash_stable (ctx , hasher) ; } } }
};
}
