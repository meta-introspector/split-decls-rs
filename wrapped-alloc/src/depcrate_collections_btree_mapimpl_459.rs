// Generated macro for impl_459 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_459 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_459"}
// Dependencies: {}
# [stable (feature = "map_values_mut" , since = "1.10.0")] impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; fn next (& mut self) -> Option < & 'a mut V > { self . inner . next () . map (| (_ , v) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn last (mut self) -> Option < & 'a mut V > { self . next_back () } }
};
}
