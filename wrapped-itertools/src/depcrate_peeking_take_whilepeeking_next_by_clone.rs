// Generated macro for peeking_next_by_clone (macro)
macro_rules! Depcrate_peeking_take_whilepeeking_next_by_clone {
() => {
// Module: crate::peeking_take_while
// Provides: {"peeking_next_by_clone"}
// Dependencies: {}
macro_rules ! peeking_next_by_clone { ([$ ($ typarm : tt) *] $ type_ : ty) => { impl <$ ($ typarm) *> PeekingNext for $ type_ { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool { let saved_state = self . clone () ; if let Some (r) = self . next () { if ! accept (& r) { * self = saved_state ; } else { return Some (r) } } None } } } }
};
}
