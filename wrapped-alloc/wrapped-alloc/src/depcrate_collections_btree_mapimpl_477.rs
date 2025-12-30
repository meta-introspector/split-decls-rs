// Generated macro for impl_477 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_477 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_477"}
// Dependencies: {}
# [stable (feature = "btree_range" , since = "1.17.0")] impl < 'a , K , V > Iterator for RangeMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . inner . next_checked () } fn last (mut self) -> Option < (& 'a K , & 'a mut V) > { self . next_back () } fn min (mut self) -> Option < (& 'a K , & 'a mut V) > where (& 'a K , & 'a mut V) : Ord , { self . next () } fn max (mut self) -> Option < (& 'a K , & 'a mut V) > where (& 'a K , & 'a mut V) : Ord , { self . next_back () } }
};
}
