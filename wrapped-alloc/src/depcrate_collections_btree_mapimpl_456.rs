// Generated macro for impl_456 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_456 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_456"}
// Dependencies: {}
# [stable (feature = "btree_range" , since = "1.17.0")] impl < 'a , K , V > Iterator for Range < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { self . inner . next_checked () } fn last (mut self) -> Option < (& 'a K , & 'a V) > { self . next_back () } fn min (mut self) -> Option < (& 'a K , & 'a V) > where (& 'a K , & 'a V) : Ord , { self . next () } fn max (mut self) -> Option < (& 'a K , & 'a V) > where (& 'a K , & 'a V) : Ord , { self . next_back () } }
};
}
