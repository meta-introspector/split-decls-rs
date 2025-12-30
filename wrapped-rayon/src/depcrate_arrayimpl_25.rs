// Generated macro for impl_25 (impl)
macro_rules! Depcrate_arrayimpl_25 {
() => {
// Module: crate::array
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'data , T : Sync + 'data , const N : usize > IntoParallelIterator for & 'data [T ; N] { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & [T] > :: into_par_iter (self) } }
};
}
