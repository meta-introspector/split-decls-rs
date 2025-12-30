// Generated macro for impl_469 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_469 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_469"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V , A : Allocator + Clone > Iterator for IntoValues < K , V , A > { type Item = V ; fn next (& mut self) -> Option < V > { self . inner . next () . map (| (_ , v) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn last (mut self) -> Option < V > { self . next_back () } }
};
}
