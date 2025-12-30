// Generated macro for impl_138 (impl)
macro_rules! Depcrate_adaptorsimpl_138 {
() => {
// Module: crate::adaptors
// Provides: {"impl_138"}
// Dependencies: {}
impl < I , F > Iterator for TakeWhileRef < '_ , I , F > where I : Iterator + Clone , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let old = self . iter . clone () ; match self . iter . next () { None => None , Some (elt) => { if (self . f) (& elt) { Some (elt) } else { * self . iter = old ; None } } } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } }
};
}
