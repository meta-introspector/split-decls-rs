// Generated macro for impl_558 (impl)
macro_rules! Depcrate_peeking_take_whileimpl_558 {
() => {
// Module: crate::peeking_take_while
// Provides: {"impl_558"}
// Dependencies: {}
impl < I , F > Iterator for PeekingTakeWhile < '_ , I , F > where I : PeekingNext , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . iter . peeking_next (& mut self . f) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } }
};
}
