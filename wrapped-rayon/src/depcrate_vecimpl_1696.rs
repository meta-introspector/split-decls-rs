// Generated macro for impl_1696 (impl)
macro_rules! Depcrate_vecimpl_1696 {
() => {
// Module: crate::vec
// Provides: {"impl_1696"}
// Dependencies: {}
impl < 'data , T : Send + 'data > IntoParallelIterator for & 'data mut Vec < T > { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & mut [T] > :: into_par_iter (self) } }
};
}
