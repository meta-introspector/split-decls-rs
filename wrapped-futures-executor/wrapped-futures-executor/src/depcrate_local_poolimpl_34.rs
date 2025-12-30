// Generated macro for impl_34 (impl)
macro_rules! Depcrate_local_poolimpl_34 {
() => {
// Module: crate::local_pool
// Provides: {"impl_34"}
// Dependencies: {}
impl < S : Stream + Unpin > Iterator for BlockingStream < S > { type Item = S :: Item ; fn next (& mut self) -> Option < Self :: Item > { LocalPool :: new () . run_until (self . stream . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
};
}
