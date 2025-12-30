// Generated macro for impl_553 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_553 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_553"}
// Dependencies: {}
impl < T , const N : usize > PeekingNext for :: core :: array :: IntoIter < T , N > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (self . as_slice () . first () ?) { true => self . next () , false => None , } } }
};
}
