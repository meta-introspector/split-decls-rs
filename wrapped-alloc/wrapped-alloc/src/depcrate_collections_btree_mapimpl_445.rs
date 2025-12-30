// Generated macro for impl_445 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_445 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_445"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > DoubleEndedIterator for Values < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a V > { self . inner . next_back () . map (| (_ , v) | v) } }
};
}
