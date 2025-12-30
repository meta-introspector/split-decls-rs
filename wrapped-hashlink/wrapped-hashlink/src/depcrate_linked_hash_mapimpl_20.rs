// Generated macro for impl_20 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_20 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_20"}
// Dependencies: {}
impl < K : Hash + Eq , V : Hash , S : BuildHasher > Hash for LinkedHashMap < K , V , S > { # [inline] fn hash < H : Hasher > (& self , h : & mut H) { for e in self . iter () { e . hash (h) ; } } }
};
}
