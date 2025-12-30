// Generated macro for impl_50 (impl)
macro_rules! Depcrate_dequeimpl_50 {
() => {
// Module: crate::deque
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
