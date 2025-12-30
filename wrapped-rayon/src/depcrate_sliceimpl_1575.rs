// Generated macro for impl_1575 (impl)
macro_rules! Depcrate_sliceimpl_1575 {
() => {
// Module: crate::slice
// Provides: {"impl_1575"}
// Dependencies: {}
impl < 'data , T : Sync > IntoParallelIterator for & 'data [T] { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { slice : self } } }
};
}
