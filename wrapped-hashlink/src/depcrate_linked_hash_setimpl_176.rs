// Generated macro for impl_176 (impl)
macro_rules! Depcrate_linked_hash_setimpl_176 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_176"}
// Dependencies: {}
impl < K > DoubleEndedIterator for Drain < '_ , K > { # [inline] fn next_back (& mut self) -> Option < K > { self . iter . next_back () . map (| (k , _) | k) } }
};
}
