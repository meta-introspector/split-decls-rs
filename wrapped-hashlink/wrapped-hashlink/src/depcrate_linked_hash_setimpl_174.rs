// Generated macro for impl_174 (impl)
macro_rules! Depcrate_linked_hash_setimpl_174 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_174"}
// Dependencies: {}
impl < K > DoubleEndedIterator for IntoIter < K > { # [inline] fn next_back (& mut self) -> Option < K > { self . iter . next_back () . map (| (k , _) | k) } }
};
}
