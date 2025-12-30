// Generated macro for impl_330 (impl)
macro_rules! Depcrate_streamimpl_330 {
() => {
// Module: crate::stream
// Provides: {"impl_330"}
// Dependencies: {}
impl < Input > Iterator for IteratorStream < Input > where Input : Iterator , { type Item = Input :: Item ; fn next (& mut self) -> Option < Input :: Item > { self . 0 . next () } }
};
}
