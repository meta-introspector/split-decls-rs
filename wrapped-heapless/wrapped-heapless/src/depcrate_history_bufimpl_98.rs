// Generated macro for impl_98 (impl)
macro_rules! Depcrate_history_bufimpl_98 {
() => {
// Module: crate::history_buf
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , T > Iterator for OldestOrdered < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
