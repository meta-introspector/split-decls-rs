// Generated macro for impl_285 (impl)
macro_rules! Depcrate_ord_setimpl_285 {
() => {
// Module: crate::ord::set
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'a , A : Eq + Hash + Ord + Clone > From < & 'a collections :: HashSet < A > > for OrdSet < A > { fn from (hash_set : & collections :: HashSet < A >) -> Self { hash_set . iter () . cloned () . collect () } }
};
}
