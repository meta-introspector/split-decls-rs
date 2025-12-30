// Generated macro for impl_287 (impl)
macro_rules! Depcrate_ord_setimpl_287 {
() => {
// Module: crate::ord::set
// Provides: {"impl_287"}
// Dependencies: {}
impl < 'a , A : Ord + Clone > From < & 'a collections :: BTreeSet < A > > for OrdSet < A > { fn from (btree_set : & collections :: BTreeSet < A >) -> Self { btree_set . iter () . cloned () . collect () } }
};
}
