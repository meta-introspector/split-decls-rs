// Generated macro for impl_1576 (impl)
macro_rules! Depcrate_syncimpl_1576 {
() => {
// Module: crate::sync
// Provides: {"impl_1576"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > Arc < T , A > { # [doc = " Allocates an `ArcInner<T>` with sufficient space for an unsized inner value."] # [inline] # [cfg (not (no_global_oom_handling))] unsafe fn allocate_for_ptr_in (ptr : * const T , alloc : & A) -> * mut ArcInner < T > { unsafe { Arc :: allocate_for_layout (Layout :: for_value_raw (ptr) , | layout | alloc . allocate (layout) , | mem | mem . with_metadata_of (ptr as * const ArcInner < T >) ,) } } # [cfg (not (no_global_oom_handling))] fn from_box_in (src : Box < T , A >) -> Arc < T , A > { unsafe { let value_size = size_of_val (& * src) ; let ptr = Self :: allocate_for_ptr_in (& * src , Box :: allocator (& src)) ; ptr :: copy_nonoverlapping ((& raw const * src) as * const u8 , (& raw mut (* ptr) . data) as * mut u8 , value_size ,) ; let (bptr , alloc) = Box :: into_raw_with_allocator (src) ; let src = Box :: from_raw_in (bptr as * mut mem :: ManuallyDrop < T > , alloc . by_ref ()) ; drop (src) ; Self :: from_ptr_in (ptr , alloc) } } }
};
}
