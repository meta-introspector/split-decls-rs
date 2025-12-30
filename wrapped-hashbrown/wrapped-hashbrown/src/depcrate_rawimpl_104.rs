// Generated macro for impl_104 (impl)
macro_rules! Depcrate_rawimpl_104 {
() => {
// Module: crate::raw
// Provides: {"impl_104"}
// Dependencies: {}
impl < T > Iterator for RawIter < T > { type Item = Bucket < T > ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < Bucket < T > > { if self . items == 0 { return None ; } let nxt = unsafe { self . iter . next_impl :: < false > () } ; debug_assert ! (nxt . is_some ()) ; self . items -= 1 ; nxt } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . items , Some (self . items)) } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { unsafe { self . iter . fold_impl (self . items , init , f) } } }
};
}
