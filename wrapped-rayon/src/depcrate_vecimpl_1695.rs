// Generated macro for impl_1695 (impl)
macro_rules! Depcrate_vecimpl_1695 {
() => {
// Module: crate::vec
// Provides: {"impl_1695"}
// Dependencies: {}
impl < 'data , T : Sync + 'data > IntoParallelIterator for & 'data Vec < T > { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & [T] > :: into_par_iter (self) } }
};
}
