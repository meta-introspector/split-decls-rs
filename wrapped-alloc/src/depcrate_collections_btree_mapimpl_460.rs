// Generated macro for impl_460 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_460 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_460"}
// Dependencies: {}
# [stable (feature = "map_values_mut" , since = "1.10.0")] impl < 'a , K , V > DoubleEndedIterator for ValuesMut < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a mut V > { self . inner . next_back () . map (| (_ , v) | v) } }
};
}
