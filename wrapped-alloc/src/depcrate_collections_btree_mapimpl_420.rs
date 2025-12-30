// Generated macro for impl_420 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_420 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_420"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K : 'a , V : 'a > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { if self . length == 0 { None } else { self . length -= 1 ; Some (unsafe { self . range . next_unchecked () }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . length , Some (self . length)) } fn last (mut self) -> Option < (& 'a K , & 'a V) > { self . next_back () } fn min (mut self) -> Option < (& 'a K , & 'a V) > where (& 'a K , & 'a V) : Ord , { self . next () } fn max (mut self) -> Option < (& 'a K , & 'a V) > where (& 'a K , & 'a V) : Ord , { self . next_back () } }
};
}
