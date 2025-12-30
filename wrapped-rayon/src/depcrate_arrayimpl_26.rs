// Generated macro for impl_26 (impl)
macro_rules! Depcrate_arrayimpl_26 {
() => {
// Module: crate::array
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'data , T : Send + 'data , const N : usize > IntoParallelIterator for & 'data mut [T ; N] { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & mut [T] > :: into_par_iter (self) } }
};
}
