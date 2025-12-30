// Generated macro for impl_734 (impl)
macro_rules! Depcrate_collections_btree_setimpl_734 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_734"}
// Dependencies: {}
impl < T : Ord , A : Allocator + Clone > BTreeSet < T , A > { fn from_sorted_iter < I : Iterator < Item = T > > (iter : I , alloc : A) -> BTreeSet < T , A > { let iter = iter . map (| k | (k , SetValZST :: default ())) ; let map = BTreeMap :: bulk_build_from_sorted_iter (iter , alloc) ; BTreeSet { map } } }
};
}
