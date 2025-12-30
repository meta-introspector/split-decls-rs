// Generated macro for RawVecInner (struct)
macro_rules! Depcrate_raw_vecRawVecInner {
() => {
// Module: crate::raw_vec
// Provides: {"RawVecInner"}
// Dependencies: {}
# [doc = " Like a `RawVec`, but only generic over the allocator, not the type."] # [doc = ""] # [doc = " As such, all the methods need the layout passed-in as a parameter."] # [doc = ""] # [doc = " Having this separation reduces the amount of code we need to monomorphize,"] # [doc = " as most operations don't need the actual type, just its layout."] # [allow (missing_debug_implementations)] struct RawVecInner < A : Allocator = Global > { ptr : Unique < u8 > , # [doc = " Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `cap` must be in the `0..=isize::MAX` range."] cap : Cap , alloc : A , }
};
}
