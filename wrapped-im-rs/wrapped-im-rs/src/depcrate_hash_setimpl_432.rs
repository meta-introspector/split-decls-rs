// Generated macro for impl_432 (impl)
macro_rules! Depcrate_hash_setimpl_432 {
() => {
// Module: crate::hash::set
// Provides: {"impl_432"}
// Dependencies: {}
impl < A , S > From < collections :: HashSet < A > > for HashSet < A , S > where A : Eq + Hash + Clone , S : BuildHasher + Default , { fn from (hash_set : collections :: HashSet < A >) -> Self { hash_set . into_iter () . collect () } }
};
}
