// Generated macro for impl_132 (impl)
macro_rules! Depcrate_setimpl_132 {
() => {
// Module: crate::set
// Provides: {"impl_132"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { self . inner . next () . map (| (k , ()) | k) } }
};
}
