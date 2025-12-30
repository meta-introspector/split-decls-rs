// Generated macro for impl_740 (impl)
macro_rules! Depcrate_collections_btree_setimpl_740 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_740"}
// Dependencies: {}
# [stable (feature = "btree_extract_if" , since = "1.91.0")] impl < T , R , F , A : Allocator + Clone > Iterator for ExtractIf < '_ , T , R , F , A > where T : PartialOrd , R : RangeBounds < T > , F : FnMut (& T) -> bool , { type Item = T ; fn next (& mut self) -> Option < T > { let pred = & mut self . pred ; let mut mapped_pred = | k : & T , _v : & mut SetValZST | pred (k) ; self . inner . next (& mut mapped_pred , self . alloc . clone ()) . map (| (k , _) | k) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
