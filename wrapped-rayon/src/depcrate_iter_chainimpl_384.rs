// Generated macro for impl_384 (impl)
macro_rules! Depcrate_iter_chainimpl_384 {
() => {
// Module: crate::iter::chain
// Provides: {"impl_384"}
// Dependencies: {}
impl < A , B > DoubleEndedIterator for ChainSeq < A , B > where A : DoubleEndedIterator , B : DoubleEndedIterator < Item = A :: Item > , { fn next_back (& mut self) -> Option < Self :: Item > { self . chain . next_back () } }
};
}
