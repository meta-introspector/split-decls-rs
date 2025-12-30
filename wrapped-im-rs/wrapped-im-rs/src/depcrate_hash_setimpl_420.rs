// Generated macro for impl_420 (impl)
macro_rules! Depcrate_hash_setimpl_420 {
() => {
// Module: crate::hash::set
// Provides: {"impl_420"}
// Dependencies: {}
impl < A > Iterator for ConsumingIter < A > where A : Hash + Eq + Clone , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| (v , _) | v . 0) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
