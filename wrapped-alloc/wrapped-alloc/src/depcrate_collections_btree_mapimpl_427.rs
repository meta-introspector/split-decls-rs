// Generated macro for impl_427 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_427 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_427"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > DoubleEndedIterator for IterMut < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . length == 0 { None } else { self . length -= 1 ; Some (unsafe { self . range . next_back_unchecked () }) } } }
};
}
