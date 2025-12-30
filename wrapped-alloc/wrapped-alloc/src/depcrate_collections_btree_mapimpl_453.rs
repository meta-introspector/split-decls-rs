// Generated macro for impl_453 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_453 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_453"}
// Dependencies: {}
# [stable (feature = "btree_extract_if" , since = "1.91.0")] impl < K , V , R , F , A : Allocator + Clone > Iterator for ExtractIf < '_ , K , V , R , F , A > where K : PartialOrd , R : RangeBounds < K > , F : FnMut (& K , & mut V) -> bool , { type Item = (K , V) ; fn next (& mut self) -> Option < (K , V) > { self . inner . next (& mut self . pred , self . alloc . clone ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
