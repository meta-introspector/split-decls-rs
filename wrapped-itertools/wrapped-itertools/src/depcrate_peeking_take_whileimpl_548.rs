// Generated macro for impl_548 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_548 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_548"}
// Dependencies: {}
impl < I > PeekingNext for PutBack < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if let Some (r) = self . next () { if ! accept (& r) { self . put_back (r) ; return None ; } Some (r) } else { None } } }
};
}
