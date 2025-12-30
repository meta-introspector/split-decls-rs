// Generated macro for impl_552 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_552 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_552"}
// Dependencies: {}
# [cfg (feature = "use_alloc")] impl < 'a > PeekingNext for :: alloc :: string :: Drain < 'a > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (& self . as_str () . chars () . next () ?) { true => self . next () , false => None , } } }
};
}
