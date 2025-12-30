// Generated macro for impl_172 (impl)
macro_rules! Depcrate_linked_hash_setimpl_172 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_172"}
// Dependencies: {}
impl < K > Iterator for IntoIter < K > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . iter . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
