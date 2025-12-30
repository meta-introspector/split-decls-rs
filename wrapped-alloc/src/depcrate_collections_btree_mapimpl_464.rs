// Generated macro for impl_464 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_464 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_464"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V , A : Allocator + Clone > Iterator for IntoKeys < K , V , A > { type Item = K ; fn next (& mut self) -> Option < K > { self . inner . next () . map (| (k , _) | k) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn last (mut self) -> Option < K > { self . next_back () } fn min (mut self) -> Option < K > where K : Ord , { self . next () } fn max (mut self) -> Option < K > where K : Ord , { self . next_back () } }
};
}
