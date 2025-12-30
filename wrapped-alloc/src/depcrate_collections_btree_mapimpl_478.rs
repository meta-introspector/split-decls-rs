// Generated macro for impl_478 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_478 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_478"}
// Dependencies: {}
# [stable (feature = "btree_range" , since = "1.17.0")] impl < 'a , K , V > DoubleEndedIterator for RangeMut < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . inner . next_back_checked () } }
};
}
