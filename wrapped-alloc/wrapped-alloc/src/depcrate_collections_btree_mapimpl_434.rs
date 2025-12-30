// Generated macro for impl_434 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_434 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_434"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , A : Allocator + Clone > Iterator for IntoIter < K , V , A > { type Item = (K , V) ; fn next (& mut self) -> Option < (K , V) > { self . dying_next () . map (unsafe { | kv | kv . into_key_val () }) } fn size_hint (& self) -> (usize , Option < usize >) { (self . length , Some (self . length)) } }
};
}
