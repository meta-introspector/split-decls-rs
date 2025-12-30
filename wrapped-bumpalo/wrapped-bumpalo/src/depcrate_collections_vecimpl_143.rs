// Generated macro for impl_143 (impl)
macro_rules! Depcrate_collections_vecimpl_143 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'a , 'bump , T > Iterator for Drain < 'a , 'bump , T > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
