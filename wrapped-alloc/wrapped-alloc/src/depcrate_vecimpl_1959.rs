// Generated macro for impl_1959 (impl)
macro_rules! Depcrate_vecimpl_1959 {
() => {
// Module: crate::vec
// Provides: {"impl_1959"}
// Dependencies: {}
# [doc = " Extend implementation that copies elements out of references before pushing them onto the Vec."] # [doc = ""] # [doc = " This implementation is specialized for slice iterators, where it uses [`copy_from_slice`] to"] # [doc = " append the entire slice at once."] # [doc = ""] # [doc = " [`copy_from_slice`]: slice::copy_from_slice"] # [cfg (not (no_global_oom_handling))] # [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a , T : Copy + 'a , A : Allocator > Extend < & 'a T > for Vec < T , A > { # [track_caller] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . spec_extend (iter . into_iter ()) } # [inline] # [track_caller] fn extend_one (& mut self , & item : & 'a T) { self . push (item) ; } # [inline] # [track_caller] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } # [inline] unsafe fn extend_one_unchecked (& mut self , & item : & 'a T) { unsafe { let len = self . len () ; ptr :: write (self . as_mut_ptr () . add (len) , item) ; self . set_len (len + 1) ; } } }
};
}
