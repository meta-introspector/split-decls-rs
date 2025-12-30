// Generated macro for impl_258 (impl)
macro_rules! Depcrate_vecimpl_258 {
() => {
// Module: crate::vec
// Provides: {"impl_258"}
// Dependencies: {}
# [doc = " Extend implementation that copies elements out of references before pushing them onto the Vec."] # [doc = ""] # [doc = " This implementation is specialized for slice iterators, where it uses [`copy_from_slice`] to"] # [doc = " append the entire slice at once."] # [doc = ""] # [doc = " [`copy_from_slice`]: slice::copy_from_slice"] # [cfg (not (no_global_oom_handling))] impl < 'a , T : Copy + 'a , A : Allocator + 'a > Extend < & 'a T > for Vec < T , A > { # [inline (always)] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { let mut iter = iter . into_iter () ; while let Some (element) = iter . next () { let len = self . len () ; if len == self . capacity () { let (lower , _) = iter . size_hint () ; self . reserve (lower . saturating_add (1)) ; } unsafe { ptr :: write (self . as_mut_ptr () . add (len) , * element) ; self . set_len (len + 1) ; } } } }
};
}
