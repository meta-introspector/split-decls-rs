// Generated macro for impl_160 (impl)
macro_rules! Depcrate_dataimpl_160 {
() => {
// Module: crate::data
// Provides: {"impl_160"}
// Dependencies: {}
impl Iterator for Iter < '_ > { type Item = u8 ; fn next (& mut self) -> Option < Self :: Item > { # [cfg (debug_assertions)] { if self . length != self . data . length () { panic ! ("NSData was mutated while iterating") ; } } self . bytes . next () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
