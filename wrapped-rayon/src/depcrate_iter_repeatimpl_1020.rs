// Generated macro for impl_1020 (impl)
macro_rules! Depcrate_iter_repeatimpl_1020 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1020"}
// Dependencies: {}
impl < T : Clone > DoubleEndedIterator for RepeatNProducer < T > { # [inline] fn next_back (& mut self) -> Option < T > { self . next () } # [inline] fn nth_back (& mut self , n : usize) -> Option < T > { self . nth (n) } }
};
}
