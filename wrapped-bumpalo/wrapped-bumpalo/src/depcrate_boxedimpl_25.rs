// Generated macro for impl_25 (impl)
macro_rules! Depcrate_boxedimpl_25 {
() => {
// Module: crate::boxed
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , I : DoubleEndedIterator + ? Sized > DoubleEndedIterator for Box < 'a , I > { fn next_back (& mut self) -> Option < I :: Item > { (* * self) . next_back () } fn nth_back (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth_back (n) } }
};
}
