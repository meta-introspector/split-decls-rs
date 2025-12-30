// Generated macro for impl_1386 (impl)
macro_rules! Depcrate_range_inclusiveimpl_1386 {
() => {
// Module: crate::range_inclusive
// Provides: {"impl_1386"}
// Dependencies: {}
# [doc = " Implemented for ranges of all primitive integer types and `char`."] impl < T > IntoParallelIterator for RangeInclusive < T > where Iter < T > : ParallelIterator , { type Item = < Iter < T > as ParallelIterator > :: Item ; type Iter = Iter < T > ; fn into_par_iter (self) -> Self :: Iter { Iter { range : self } } }
};
}
