// Generated macro for impl_431 (impl)
macro_rules! Depcrate_hash_setimpl_431 {
() => {
// Module: crate::hash::set
// Provides: {"impl_431"}
// Dependencies: {}
impl < 'a , A , S > From < & 'a Vector < A > > for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn from (vector : & Vector < A >) -> Self { vector . iter () . cloned () . collect () } }
};
}
