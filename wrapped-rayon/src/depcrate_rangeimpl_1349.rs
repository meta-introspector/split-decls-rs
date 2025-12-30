// Generated macro for impl_1349 (impl)
macro_rules! Depcrate_rangeimpl_1349 {
() => {
// Module: crate::range
// Provides: {"impl_1349"}
// Dependencies: {}
# [doc = " Implemented for ranges of all primitive integer types and `char`."] impl < T > IntoParallelIterator for Range < T > where Iter < T > : ParallelIterator , { type Item = < Iter < T > as ParallelIterator > :: Item ; type Iter = Iter < T > ; fn into_par_iter (self) -> Self :: Iter { Iter { range : self } } }
};
}
