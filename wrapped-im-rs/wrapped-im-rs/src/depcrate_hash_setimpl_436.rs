// Generated macro for impl_436 (impl)
macro_rules! Depcrate_hash_setimpl_436 {
() => {
// Module: crate::hash::set
// Provides: {"impl_436"}
// Dependencies: {}
impl < 'a , A , S > From < & 'a OrdSet < A > > for HashSet < A , S > where A : Ord + Hash + Eq + Clone , S : BuildHasher + Default , { fn from (ordset : & OrdSet < A >) -> Self { ordset . into_iter () . cloned () . collect () } }
};
}
