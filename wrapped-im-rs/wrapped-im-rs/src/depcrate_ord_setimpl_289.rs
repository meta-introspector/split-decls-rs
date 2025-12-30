// Generated macro for impl_289 (impl)
macro_rules! Depcrate_ord_setimpl_289 {
() => {
// Module: crate::ord::set
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'a , A : Hash + Eq + Ord + Clone , S : BuildHasher > From < & 'a HashSet < A , S > > for OrdSet < A > { fn from (hashset : & HashSet < A , S >) -> Self { hashset . into_iter () . cloned () . collect () } }
};
}
