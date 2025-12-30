// Generated macro for impl_181 (impl)
macro_rules! Depcrate_adaptorsimpl_181 {
() => {
// Module: crate::adaptors
// Provides: {"impl_181"}
// Dependencies: {}
impl < I , F > Iterator for Positions < I , F > where I : Iterator , F : FnMut (I :: Item) -> bool , { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . find_map (| (count , val) | f (val) . then_some (count)) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } fn fold < B , G > (self , init : B , mut func : G) -> B where G : FnMut (B , Self :: Item) -> B , { let mut f = self . f ; self . iter . fold (init , | mut acc , (count , val) | { if f (val) { acc = func (acc , count) ; } acc }) } }
};
}
