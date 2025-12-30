// Generated macro for impl_27 (impl)
macro_rules! Depcrate_arrayimpl_27 {
() => {
// Module: crate::array
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : Send , const N : usize > IntoParallelIterator for [T ; N] { type Item = T ; type Iter = IntoIter < T , N > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { array : self } } }
};
}
