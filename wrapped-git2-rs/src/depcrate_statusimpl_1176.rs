// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_statusimpl_1176 {
() => {
// Module: crate::status
// Provides: {"impl_1176"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for StatusIter < 'a > { fn next_back (& mut self) -> Option < StatusEntry < 'a > > { self . range . next_back () . and_then (| i | self . statuses . get (i)) } }
};
}
