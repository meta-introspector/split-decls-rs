// Generated macro for impl_26 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_26 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_26"}
// Dependencies: {}
impl < K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Clone > Clone for LinkedHashMap < K , V , S > { # [inline] fn clone (& self) -> Self { let mut map = Self :: with_hasher (self . hash_builder . clone ()) ; map . extend (self . iter () . map (| (k , v) | (k . clone () , v . clone ()))) ; map } }
};
}
