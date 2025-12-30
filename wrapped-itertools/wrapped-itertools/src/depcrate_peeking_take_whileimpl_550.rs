// Generated macro for impl_550 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_550 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_550"}
// Dependencies: {}
# [cfg (feature = "use_alloc")] impl < T > PeekingNext for :: alloc :: vec :: IntoIter < T > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (self . as_slice () . first () ?) { true => self . next () , false => None , } } }
};
}
