// Generated macro for impl_1107 (impl)
macro_rules! Depcrate_iter_takeimpl_1107 {
() => {
// Module: crate::iter::take
// Provides: {"impl_1107"}
// Dependencies: {}
impl < I > Take < I > where I : IndexedParallelIterator , { # [doc = " Creates a new `Take` iterator."] pub (super) fn new (base : I , n : usize) -> Self { let n = Ord :: min (base . len () , n) ; Take { base , n } } }
};
}
