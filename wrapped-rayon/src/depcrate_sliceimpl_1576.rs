// Generated macro for impl_1576 (impl)
macro_rules! Depcrate_sliceimpl_1576 {
() => {
// Module: crate::slice
// Provides: {"impl_1576"}
// Dependencies: {}
impl < 'data , T : Sync > IntoParallelIterator for & 'data Box < [T] > { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { slice : self } } }
};
}
