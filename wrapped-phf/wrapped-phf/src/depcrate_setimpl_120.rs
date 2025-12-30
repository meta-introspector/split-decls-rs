// Generated macro for impl_120 (impl)
macro_rules! Depcrate_setimpl_120 {
() => {
// Module: crate::set
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
