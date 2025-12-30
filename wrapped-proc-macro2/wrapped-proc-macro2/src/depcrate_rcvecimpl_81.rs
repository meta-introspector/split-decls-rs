// Generated macro for impl_81 (impl)
macro_rules! Depcrate_rcvecimpl_81 {
() => {
// Module: crate::rcvec
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > IntoIterator for RcVecBuilder < T > { type Item = T ; type IntoIter = RcVecIntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { RcVecIntoIter { inner : self . inner . into_iter () , } } }
};
}
