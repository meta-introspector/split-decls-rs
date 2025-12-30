// Generated macro for impl_549 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_549 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_549"}
// Dependencies: {}
# [cfg (feature = "use_alloc")] impl < I > PeekingNext for PutBackN < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if let Some (r) = self . next () { if ! accept (& r) { self . put_back (r) ; return None ; } Some (r) } else { None } } }
};
}
