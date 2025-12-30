// Generated macro for impl_492 (impl)
macro_rules! Depcrate_sparseimpl_492 {
() => {
// Module: crate::sparse
// Provides: {"impl_492"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a SparseSet { type Item = & 'a usize ; type IntoIter = slice :: Iter < 'a , usize > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
