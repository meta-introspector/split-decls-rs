// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_rcimpl_1223 {
() => {
// Module: crate::rc
// Provides: {"impl_1223"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > Rc < T , A > { # [doc = " Allocates an `RcInner<T>` with sufficient space for an unsized inner value"] # [cfg (not (no_global_oom_handling))] unsafe fn allocate_for_ptr_in (ptr : * const T , alloc : & A) -> * mut RcInner < T > { unsafe { Rc :: < T > :: allocate_for_layout (Layout :: for_value_raw (ptr) , | layout | alloc . allocate (layout) , | mem | mem . with_metadata_of (ptr as * const RcInner < T >) ,) } } # [cfg (not (no_global_oom_handling))] fn from_box_in (src : Box < T , A >) -> Rc < T , A > { unsafe { let value_size = size_of_val (& * src) ; let ptr = Self :: allocate_for_ptr_in (& * src , Box :: allocator (& src)) ; ptr :: copy_nonoverlapping ((& raw const * src) as * const u8 , (& raw mut (* ptr) . value) as * mut u8 , value_size ,) ; let (bptr , alloc) = Box :: into_raw_with_allocator (src) ; let src = Box :: from_raw_in (bptr as * mut mem :: ManuallyDrop < T > , alloc . by_ref ()) ; drop (src) ; Self :: from_ptr_in (ptr , alloc) } } }
};
}
