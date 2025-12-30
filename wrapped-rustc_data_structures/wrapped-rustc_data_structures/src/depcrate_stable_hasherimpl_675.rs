// Generated macro for impl_675 (impl)
macro_rules! Depcrate_stable_hasherimpl_675 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_675"}
// Dependencies: {}
impl < K , V , R , CTX > HashStable < CTX > for indexmap :: IndexMap < K , V , R > where K : HashStable < CTX > + Eq + Hash , V : HashStable < CTX > , R : BuildHasher , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for kv in self { kv . hash_stable (ctx , hasher) ; } } }
};
}
