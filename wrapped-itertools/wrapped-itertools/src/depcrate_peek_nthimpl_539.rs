// Generated macro for impl_539 (impl)
macro_rules! Depcrate_peek_nthimpl_539 {
() => {
// Module: crate::peek_nth
// Provides: {"impl_539"}
// Dependencies: {}
impl < I > PeekingNext for PeekNth < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { self . peek () . filter (| item | accept (item)) ? ; self . next () } }
};
}
