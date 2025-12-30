// Generated macro for impl_554 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_554 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_554"}
// Dependencies: {}
impl < T : Clone > PeekingNext for RepeatN < T > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { let r = self . elt . as_ref () ? ; if ! accept (r) { return None ; } self . next () } }
};
}
