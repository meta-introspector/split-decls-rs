// Generated macro for impl_144 (impl)
macro_rules! Depcrate_collections_vecimpl_144 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_144"}
// Dependencies: {}
impl < 'a , 'bump , T > DoubleEndedIterator for Drain < 'a , 'bump , T > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } }
};
}
