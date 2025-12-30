// Generated macro for impl_288 (impl)
macro_rules! Depcrate_ord_setimpl_288 {
() => {
// Module: crate::ord::set
// Provides: {"impl_288"}
// Dependencies: {}
impl < A : Hash + Eq + Ord + Clone , S : BuildHasher > From < HashSet < A , S > > for OrdSet < A > { fn from (hashset : HashSet < A , S >) -> Self { hashset . into_iter () . collect () } }
};
}
