// Generated macro for impl_429 (impl)
macro_rules! Depcrate_hash_setimpl_429 {
() => {
// Module: crate::hash::set
// Provides: {"impl_429"}
// Dependencies: {}
impl < 'a , A , S > From < & 'a Vec < A > > for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn from (vec : & Vec < A >) -> Self { vec . iter () . cloned () . collect () } }
};
}
