// Generated macro for impl_426 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_426 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_426"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . length == 0 { None } else { self . length -= 1 ; Some (unsafe { self . range . next_unchecked () }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . length , Some (self . length)) } fn last (mut self) -> Option < (& 'a K , & 'a mut V) > { self . next_back () } fn min (mut self) -> Option < (& 'a K , & 'a mut V) > where (& 'a K , & 'a mut V) : Ord , { self . next () } fn max (mut self) -> Option < (& 'a K , & 'a mut V) > where (& 'a K , & 'a mut V) : Ord , { self . next_back () } }
};
}
