// Generated macro for impl_439 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_439 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_439"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > DoubleEndedIterator for Keys < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a K > { self . inner . next_back () . map (| (k , _) | k) } }
};
}
