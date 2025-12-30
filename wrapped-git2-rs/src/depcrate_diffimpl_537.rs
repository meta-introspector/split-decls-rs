// Generated macro for impl_537 (impl)
macro_rules! Depcrate_diffimpl_537 {
() => {
// Module: crate::diff
// Provides: {"impl_537"}
// Dependencies: {}
impl < 'diff > DoubleEndedIterator for Deltas < 'diff > { fn next_back (& mut self) -> Option < DiffDelta < 'diff > > { self . range . next_back () . and_then (| i | self . diff . get_delta (i)) } }
};
}
