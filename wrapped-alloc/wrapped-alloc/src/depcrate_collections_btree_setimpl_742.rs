// Generated macro for impl_742 (impl)
macro_rules! Depcrate_collections_btree_setimpl_742 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_742"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , A : Allocator + Clone > Extend < T > for BTreeSet < T , A > { # [inline] fn extend < Iter : IntoIterator < Item = T > > (& mut self , iter : Iter) { iter . into_iter () . for_each (move | elem | { self . insert (elem) ; }) ; } # [inline] fn extend_one (& mut self , elem : T) { self . insert (elem) ; } }
};
}
