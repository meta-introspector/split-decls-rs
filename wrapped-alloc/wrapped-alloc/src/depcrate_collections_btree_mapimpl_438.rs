// Generated macro for impl_438 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_438 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_438"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < & 'a K > { self . inner . next () . map (| (k , _) | k) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn last (mut self) -> Option < & 'a K > { self . next_back () } fn min (mut self) -> Option < & 'a K > where & 'a K : Ord , { self . next () } fn max (mut self) -> Option < & 'a K > where & 'a K : Ord , { self . next_back () } }
};
}
