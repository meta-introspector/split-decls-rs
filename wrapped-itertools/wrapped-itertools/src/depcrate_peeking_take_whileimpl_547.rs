// Generated macro for impl_547 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_547 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_547"}
// Dependencies: {}
impl < I > PeekingNext for Peekable < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if let Some (r) = self . peek () { if ! accept (r) { return None ; } } self . next () } }
};
}
