// Generated macro for impl_1577 (impl)
macro_rules! Depcrate_sliceimpl_1577 {
() => {
// Module: crate::slice
// Provides: {"impl_1577"}
// Dependencies: {}
impl < 'data , T : Send > IntoParallelIterator for & 'data mut [T] { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { slice : self } } }
};
}
