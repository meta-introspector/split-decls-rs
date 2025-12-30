// Generated macro for impl_286 (impl)
macro_rules! Depcrate_ord_setimpl_286 {
() => {
// Module: crate::ord::set
// Provides: {"impl_286"}
// Dependencies: {}
impl < A : Ord + Clone > From < collections :: BTreeSet < A > > for OrdSet < A > { fn from (btree_set : collections :: BTreeSet < A >) -> Self { btree_set . into_iter () . collect () } }
};
}
