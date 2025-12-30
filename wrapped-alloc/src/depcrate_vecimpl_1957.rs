// Generated macro for impl_1957 (impl)
macro_rules! Depcrate_vecimpl_1957 {
() => {
// Module: crate::vec
// Provides: {"impl_1957"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Extend < T > for Vec < T , A > { # [inline] # [track_caller] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { < Self as SpecExtend < T , I :: IntoIter > > :: spec_extend (self , iter . into_iter ()) } # [inline] # [track_caller] fn extend_one (& mut self , item : T) { self . push (item) ; } # [inline] # [track_caller] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } # [inline] unsafe fn extend_one_unchecked (& mut self , item : T) { unsafe { let len = self . len () ; ptr :: write (self . as_mut_ptr () . add (len) , item) ; self . set_len (len + 1) ; } } }
};
}
