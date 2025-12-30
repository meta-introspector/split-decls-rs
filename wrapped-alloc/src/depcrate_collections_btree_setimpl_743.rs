// Generated macro for impl_743 (impl)
macro_rules! Depcrate_collections_btree_setimpl_743 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_743"}
// Dependencies: {}
# [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a , T : 'a + Ord + Copy , A : Allocator + Clone > Extend < & 'a T > for BTreeSet < T , A > { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] fn extend_one (& mut self , & elem : & 'a T) { self . insert (elem) ; } }
};
}
