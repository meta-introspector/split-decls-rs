// Generated macro for impl_32 (impl)
macro_rules! Depcrate_iter_setimpl_32 {
() => {
// Module: crate::iter_set
// Provides: {"impl_32"}
// Dependencies: {}
impl < K : Eq + Hash > Iterator for OwningIter < K > { type Item = K ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , _) | k) } }
};
}
