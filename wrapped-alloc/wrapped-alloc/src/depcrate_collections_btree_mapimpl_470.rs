// Generated macro for impl_470 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_470 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_470"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V , A : Allocator + Clone > DoubleEndedIterator for IntoValues < K , V , A > { fn next_back (& mut self) -> Option < V > { self . inner . next_back () . map (| (_ , v) | v) } }
};
}
