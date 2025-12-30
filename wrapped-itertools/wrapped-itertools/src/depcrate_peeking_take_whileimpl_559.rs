// Generated macro for impl_559 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_559 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_559"}
// Dependencies: {}
impl < I , F > PeekingNext for PeekingTakeWhile < '_ , I , F > where I : PeekingNext , F : FnMut (& I :: Item) -> bool , { fn peeking_next < G > (& mut self , g : G) -> Option < Self :: Item > where G : FnOnce (& Self :: Item) -> bool , { let f = & mut self . f ; self . iter . peeking_next (| r | f (r) && g (r)) } }
};
}
