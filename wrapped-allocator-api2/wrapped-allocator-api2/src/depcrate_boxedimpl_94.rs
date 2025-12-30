// Generated macro for impl_94 (impl)
macro_rules! Depcrate_boxedimpl_94 {
() => {
// Module: crate::boxed
// Provides: {"impl_94"}
// Dependencies: {}
impl < I : DoubleEndedIterator + ? Sized , A : Allocator > DoubleEndedIterator for Box < I , A > { # [inline (always)] fn next_back (& mut self) -> Option < I :: Item > { (* * self) . next_back () } # [inline (always)] fn nth_back (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth_back (n) } }
};
}
