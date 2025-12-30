// Generated macro for impl_507 (impl)
macro_rules! Depcrate_multipeek_implimpl_507 {
() => {
// Module: crate::multipeek_impl
// Provides: {"impl_507"}
// Dependencies: {}
impl < I > PeekingNext for MultiPeek < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if self . buf . is_empty () { if let Some (r) = self . peek () { if ! accept (r) { return None ; } } } else if let Some (r) = self . buf . front () { if ! accept (r) { return None ; } } self . next () } }
};
}
