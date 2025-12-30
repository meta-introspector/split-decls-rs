// Generated macro for impl_1578 (impl)
macro_rules! Depcrate_syncimpl_1578 {
() => {
// Module: crate::sync
// Provides: {"impl_1578"}
// Dependencies: {}
impl < T , A : Allocator > Arc < [T] , A > { # [doc = " Allocates an `ArcInner<[T]>` with the given length."] # [inline] # [cfg (not (no_global_oom_handling))] unsafe fn allocate_for_slice_in (len : usize , alloc : & A) -> * mut ArcInner < [T] > { unsafe { Arc :: allocate_for_layout (Layout :: array :: < T > (len) . unwrap () , | layout | alloc . allocate (layout) , | mem | ptr :: slice_from_raw_parts_mut (mem . cast :: < T > () , len) as * mut ArcInner < [T] > ,) } } }
};
}
