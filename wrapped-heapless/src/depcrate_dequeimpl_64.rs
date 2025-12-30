// Generated macro for impl_64 (impl)
macro_rules! Depcrate_dequeimpl_64 {
() => {
// Module: crate::deque
// Provides: {"impl_64"}
// Dependencies: {}
impl < T , const N : usize > IntoIterator for Deque < T , N > { type Item = T ; type IntoIter = IntoIter < T , N > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { deque : self } } }
};
}
