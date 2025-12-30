// Generated macro for impl_382 (impl)
macro_rules! Depcrate_iter_chainimpl_382 {
() => {
// Module: crate::iter::chain
// Provides: {"impl_382"}
// Dependencies: {}
impl < A , B > Iterator for ChainSeq < A , B > where A : Iterator , B : Iterator < Item = A :: Item > , { type Item = A :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . chain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . chain . size_hint () } }
};
}
