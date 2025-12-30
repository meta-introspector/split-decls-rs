// Generated macro for impl_169 (impl)
macro_rules! Depcrate_vec_spliceimpl_169 {
() => {
// Module: crate::vec::splice
// Provides: {"impl_169"}
// Dependencies: {}
impl < I : Iterator , A : Allocator > DoubleEndedIterator for Splice < '_ , I , A > { # [inline (always)] fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
};
}
