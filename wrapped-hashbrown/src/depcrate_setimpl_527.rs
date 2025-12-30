// Generated macro for impl_527 (impl)
macro_rules! Depcrate_setimpl_527 {
() => {
// Module: crate::set
// Provides: {"impl_527"}
// Dependencies: {}
impl < K , F , A : Allocator > Iterator for ExtractIf < '_ , K , F , A > where F : FnMut (& K) -> bool , { type Item = K ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next (| & mut (ref k , ()) | (self . f) (k)) . map (| (k , ()) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . inner . iter . size_hint () . 1) } }
};
}
