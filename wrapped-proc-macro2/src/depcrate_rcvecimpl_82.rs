// Generated macro for impl_82 (impl)
macro_rules! Depcrate_rcvecimpl_82 {
() => {
// Module: crate::rcvec
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > Iterator for RcVecIntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
