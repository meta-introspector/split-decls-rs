// Generated macro for impl_422 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_422 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_422"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K : 'a , V : 'a > DoubleEndedIterator for Iter < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { if self . length == 0 { None } else { self . length -= 1 ; Some (unsafe { self . range . next_back_unchecked () }) } } }
};
}
