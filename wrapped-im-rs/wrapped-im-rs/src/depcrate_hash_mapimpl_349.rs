// Generated macro for impl_349 (impl)
macro_rules! Depcrate_hash_mapimpl_349 {
() => {
// Module: crate::hash::map
// Provides: {"impl_349"}
// Dependencies: {}
impl < A > Iterator for ConsumingIter < A > where A : HashValue + Clone , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| (p , _) | p) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
