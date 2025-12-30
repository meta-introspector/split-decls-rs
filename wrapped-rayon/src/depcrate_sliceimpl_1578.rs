// Generated macro for impl_1578 (impl)
macro_rules! Depcrate_sliceimpl_1578 {
() => {
// Module: crate::slice
// Provides: {"impl_1578"}
// Dependencies: {}
impl < 'data , T : Send > IntoParallelIterator for & 'data mut Box < [T] > { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { slice : self } } }
};
}
