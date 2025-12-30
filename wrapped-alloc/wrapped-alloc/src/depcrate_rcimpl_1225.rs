// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_rcimpl_1225 {
() => {
// Module: crate::rc
// Provides: {"impl_1225"}
// Dependencies: {}
impl < T , A : Allocator > Rc < [T] , A > { # [doc = " Allocates an `RcInner<[T]>` with the given length."] # [inline] # [cfg (not (no_global_oom_handling))] unsafe fn allocate_for_slice_in (len : usize , alloc : & A) -> * mut RcInner < [T] > { unsafe { Rc :: < [T] > :: allocate_for_layout (Layout :: array :: < T > (len) . unwrap () , | layout | alloc . allocate (layout) , | mem | ptr :: slice_from_raw_parts_mut (mem . cast :: < T > () , len) as * mut RcInner < [T] > ,) } } }
};
}
