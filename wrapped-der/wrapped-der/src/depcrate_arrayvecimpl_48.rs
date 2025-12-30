// Generated macro for impl_48 (impl)
macro_rules! Depcrate_arrayvecimpl_48 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { match self . elements . get (self . position) { Some (Some (res)) => { self . position = self . position . checked_add (1) ? ; Some (res) } _ => None , } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . elements . len () . saturating_sub (self . position) ; (len , Some (len)) } }
};
}
