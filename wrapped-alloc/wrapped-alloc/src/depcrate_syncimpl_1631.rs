// Generated macro for impl_1631 (impl)
macro_rules! Depcrate_syncimpl_1631 {
() => {
// Module: crate::sync
// Provides: {"impl_1631"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl < T , A : Allocator + Clone > From < Vec < T , A > > for Arc < [T] , A > { # [doc = " Allocates a reference-counted slice and moves `v`'s items into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let unique: Vec<i32> = vec![1, 2, 3];"] # [doc = " let shared: Arc<[i32]> = Arc::from(unique);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : Vec < T , A >) -> Arc < [T] , A > { unsafe { let (vec_ptr , len , cap , alloc) = v . into_raw_parts_with_alloc () ; let rc_ptr = Self :: allocate_for_slice_in (len , & alloc) ; ptr :: copy_nonoverlapping (vec_ptr , (& raw mut (* rc_ptr) . data) as * mut T , len) ; let _ = Vec :: from_raw_parts_in (vec_ptr , 0 , cap , & alloc) ; Self :: from_ptr_in (rc_ptr , alloc) } } }
};
}
