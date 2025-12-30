// Generated macro for impl_576 (impl)
macro_rules! Depcrate_sorted_mapimpl_576 {
() => {
// Module: crate::sorted_map
// Provides: {"impl_576"}
// Dependencies: {}
impl < K : HashStable < CTX > + StableOrd , V : HashStable < CTX > , CTX > HashStable < CTX > for SortedMap < K , V > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . data . hash_stable (ctx , hasher) ; } }
};
}
