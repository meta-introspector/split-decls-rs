// Generated macro for impl_428 (impl)
macro_rules! Depcrate_hash_setimpl_428 {
() => {
// Module: crate::hash::set
// Provides: {"impl_428"}
// Dependencies: {}
impl < A , S > From < Vec < A > > for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn from (vec : Vec < A >) -> Self { vec . into_iter () . collect () } }
};
}
