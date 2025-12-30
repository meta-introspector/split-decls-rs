// Generated macro for impl_536 (impl)
macro_rules! Depcrate_diffimpl_536 {
() => {
// Module: crate::diff
// Provides: {"impl_536"}
// Dependencies: {}
impl < 'diff > Iterator for Deltas < 'diff > { type Item = DiffDelta < 'diff > ; fn next (& mut self) -> Option < DiffDelta < 'diff > > { self . range . next () . and_then (| i | self . diff . get_delta (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
