// Generated macro for impl_676 (impl)
macro_rules! Depcrate_stable_hasherimpl_676 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_676"}
// Dependencies: {}
impl < K , R , CTX > HashStable < CTX > for indexmap :: IndexSet < K , R > where K : HashStable < CTX > + Eq + Hash , R : BuildHasher , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for key in self { key . hash_stable (ctx , hasher) ; } } }
};
}
