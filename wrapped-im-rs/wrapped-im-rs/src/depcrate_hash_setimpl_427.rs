// Generated macro for impl_427 (impl)
macro_rules! Depcrate_hash_setimpl_427 {
() => {
// Module: crate::hash::set
// Provides: {"impl_427"}
// Dependencies: {}
impl < 'a , A , S > From < & 'a [A] > for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn from (slice : & 'a [A]) -> Self { slice . iter () . cloned () . collect () } }
};
}
