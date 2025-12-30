// Generated macro for impl_733 (impl)
macro_rules! Depcrate_collections_btree_setimpl_733 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_733"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord > FromIterator < T > for BTreeSet < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> BTreeSet < T > { let mut inputs : Vec < _ > = iter . into_iter () . collect () ; if inputs . is_empty () { return BTreeSet :: new () ; } inputs . sort () ; BTreeSet :: from_sorted_iter (inputs . into_iter () , Global) } }
};
}
