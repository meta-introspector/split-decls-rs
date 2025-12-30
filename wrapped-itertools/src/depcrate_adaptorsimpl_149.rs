// Generated macro for impl_149 (impl)
macro_rules! Depcrate_adaptorsimpl_149 {
() => {
// Module: crate::adaptors
// Provides: {"impl_149"}
// Dependencies: {}
impl < I : Iterator > Iterator for Tuple1Combination < I > { type Item = (I :: Item ,) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| x | (x ,)) } fn size_hint (& self) -> SizeHint { self . iter . size_hint () } fn count (self) -> usize { self . iter . count () } fn fold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . iter . map (| x | (x ,)) . fold (init , f) } }
};
}
