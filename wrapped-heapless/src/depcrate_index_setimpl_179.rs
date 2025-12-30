// Generated macro for impl_179 (impl)
macro_rules! Depcrate_index_setimpl_179 {
() => {
// Module: crate::index_set
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (k , _) | k) } }
};
}
