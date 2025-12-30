// Generated macro for impl_1037 (impl)
macro_rules! Depcrate_iter_skipimpl_1037 {
() => {
// Module: crate::iter::skip
// Provides: {"impl_1037"}
// Dependencies: {}
impl < I > Skip < I > where I : IndexedParallelIterator , { # [doc = " Creates a new `Skip` iterator."] pub (super) fn new (base : I , n : usize) -> Self { let n = Ord :: min (base . len () , n) ; Skip { base , n } } }
};
}
