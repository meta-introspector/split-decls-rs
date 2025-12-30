// Generated macro for impl_430 (impl)
macro_rules! Depcrate_hash_setimpl_430 {
() => {
// Module: crate::hash::set
// Provides: {"impl_430"}
// Dependencies: {}
impl < A , S > From < Vector < A > > for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn from (vector : Vector < A >) -> Self { vector . into_iter () . collect () } }
};
}
