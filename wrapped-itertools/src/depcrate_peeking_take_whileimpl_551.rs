// Generated macro for impl_551 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_551 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_551"}
// Dependencies: {}
# [cfg (feature = "use_alloc")] impl < 'a , T > PeekingNext for :: alloc :: vec :: Drain < 'a , T > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (self . as_slice () . first () ?) { true => self . next () , false => None , } } }
};
}
