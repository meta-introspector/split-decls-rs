// Generated macro for impl_424 (impl)
macro_rules! Depcrate_hash_setimpl_424 {
() => {
// Module: crate::hash::set
// Provides: {"impl_424"}
// Dependencies: {}
impl < 'a , A , S > IntoIterator for & 'a HashSet < A , S > where A : Hash + Eq , S : BuildHasher , { type Item = & 'a A ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
