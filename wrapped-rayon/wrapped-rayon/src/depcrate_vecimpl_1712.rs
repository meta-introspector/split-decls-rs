// Generated macro for impl_1712 (impl)
macro_rules! Depcrate_vecimpl_1712 {
() => {
// Module: crate::vec
// Provides: {"impl_1712"}
// Dependencies: {}
impl < 'data , T : 'data > Iterator for SliceDrain < 'data , T > { type Item = T ; fn next (& mut self) -> Option < T > { let ptr : * const T = self . iter . next () ? ; Some (unsafe { ptr :: read (ptr) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn count (self) -> usize { self . iter . len () } }
};
}
