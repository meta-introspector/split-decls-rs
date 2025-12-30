// Generated macro for impl_139 (impl)
macro_rules! Depcrate_rawimpl_139 {
() => {
// Module: crate::raw
// Provides: {"impl_139"}
// Dependencies: {}
impl < T , A : Allocator > RawExtractIf < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub (crate) fn next < F > (& mut self , mut f : F) -> Option < T > where F : FnMut (& mut T) -> bool , { unsafe { for item in & mut self . iter { if f (item . as_mut ()) { return Some (self . table . remove (item) . 0) ; } } } None } }
};
}
