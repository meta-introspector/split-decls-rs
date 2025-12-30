// Generated macro for impl_444 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_444 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_444"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; fn next (& mut self) -> Option < & 'a V > { self . inner . next () . map (| (_ , v) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn last (mut self) -> Option < & 'a V > { self . next_back () } }
};
}
