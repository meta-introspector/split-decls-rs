// Generated macro for impl_152 (impl)
macro_rules! Depcrate_arcimpl_152 {
() => {
// Module: crate::arc
// Provides: {"impl_152"}
// Dependencies: {}
impl < T > Arc < T > { # [doc = " Reconstruct the Arc<T> from a raw pointer obtained from into_raw()"] # [doc = ""] # [doc = " Note: This raw pointer will be offset in the allocation and must be preceded"] # [doc = " by the atomic count."] # [doc = ""] # [doc = " It is recommended to use OffsetArc for this"] # [inline] pub (crate) unsafe fn from_raw (ptr : * const T) -> Self { unsafe { let ptr = (ptr as * const u8) . sub (offset_of ! (ArcInner < T >, data)) ; Arc { p : ptr :: NonNull :: new_unchecked (ptr as * mut ArcInner < T >) , phantom : PhantomData } } } }
};
}
