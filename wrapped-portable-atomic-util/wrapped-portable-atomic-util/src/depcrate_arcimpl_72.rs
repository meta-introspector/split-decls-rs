// Generated macro for impl_72 (impl)
macro_rules! Depcrate_arcimpl_72 {
() => {
// Module: crate::arc
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl < T > From < Vec < T > > for Arc < [T] > { # [doc = " Allocates a reference-counted slice and moves `v`'s items into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let unique: Vec<i32> = vec![1, 2, 3];"] # [doc = " let shared: Arc<[i32]> = Arc::from(unique);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : Vec < T >) -> Self { unsafe { let len = v . len () ; let cap = v . capacity () ; let vec_ptr = mem :: ManuallyDrop :: new (v) . as_mut_ptr () ; let mut arc = Self :: new_uninit_slice (len) ; let data = Arc :: get_mut_unchecked (& mut arc) ; ptr :: copy_nonoverlapping (vec_ptr , data . as_mut_ptr () as * mut T , len) ; let _ = Vec :: from_raw_parts (vec_ptr , 0 , cap) ; arc . assume_init () } } }
};
}
