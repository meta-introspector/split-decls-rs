// Generated macro for impl_284 (impl)
macro_rules! Depcrate_ord_setimpl_284 {
() => {
// Module: crate::ord::set
// Provides: {"impl_284"}
// Dependencies: {}
impl < A : Eq + Hash + Ord + Clone > From < collections :: HashSet < A > > for OrdSet < A > { fn from (hash_set : collections :: HashSet < A >) -> Self { hash_set . into_iter () . collect () } }
};
}
