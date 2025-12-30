// Generated macro for impl_436 (impl)
macro_rules! Depcrate_sparseimpl_436 {
() => {
// Module: crate::sparse
// Provides: {"impl_436"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a SparseSet { type Item = & 'a usize ; type IntoIter = slice :: Iter < 'a , usize > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
