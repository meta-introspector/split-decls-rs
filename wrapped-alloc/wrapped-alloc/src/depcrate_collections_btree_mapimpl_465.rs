// Generated macro for impl_465 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_465 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_465"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V , A : Allocator + Clone > DoubleEndedIterator for IntoKeys < K , V , A > { fn next_back (& mut self) -> Option < K > { self . inner . next_back () . map (| (k , _) | k) } }
};
}
