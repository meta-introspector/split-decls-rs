// Generated macro for impl_435 (impl)
macro_rules! Depcrate_hash_setimpl_435 {
() => {
// Module: crate::hash::set
// Provides: {"impl_435"}
// Dependencies: {}
impl < A , S > From < OrdSet < A > > for HashSet < A , S > where A : Ord + Hash + Eq + Clone , S : BuildHasher + Default , { fn from (ordset : OrdSet < A >) -> Self { ordset . into_iter () . collect () } }
};
}
