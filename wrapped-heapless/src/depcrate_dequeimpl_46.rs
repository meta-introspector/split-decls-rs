// Generated macro for impl_46 (impl)
macro_rules! Depcrate_dequeimpl_46 {
() => {
// Module: crate::deque
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
