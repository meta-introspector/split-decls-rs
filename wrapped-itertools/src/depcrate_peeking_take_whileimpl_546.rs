// Generated macro for impl_546 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_546 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_546"}
// Dependencies: {}
impl < I > PeekingNext for & mut I where I : PeekingNext , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { (* self) . peeking_next (accept) } }
};
}
