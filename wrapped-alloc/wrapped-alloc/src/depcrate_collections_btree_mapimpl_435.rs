// Generated macro for impl_435 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_435 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_435"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , A : Allocator + Clone > DoubleEndedIterator for IntoIter < K , V , A > { fn next_back (& mut self) -> Option < (K , V) > { self . dying_next_back () . map (unsafe { | kv | kv . into_key_val () }) } }
};
}
