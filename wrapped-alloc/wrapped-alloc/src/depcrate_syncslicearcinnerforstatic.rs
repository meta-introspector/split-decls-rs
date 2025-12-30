// Generated macro for SliceArcInnerForStatic (struct)
macro_rules! Depcrate_syncSliceArcInnerForStatic {
() => {
// Module: crate::sync
// Provides: {"SliceArcInnerForStatic"}
// Dependencies: {}
# [doc = " Struct to hold the static `ArcInner` used for empty `Arc<str/CStr/[T]>` as"] # [doc = " returned by `Default::default`."] # [doc = ""] # [doc = " Layout notes:"] # [doc = " * `repr(align(16))` so we can use it for `[T]` with `align_of::<T>() <= 16`."] # [doc = " * `repr(C)` so `inner` is at offset 0 (and thus guaranteed to actually be aligned to 16)."] # [doc = " * `[u8; 1]` (to be initialized with 0) so it can be used for `Arc<CStr>`."] # [repr (C , align (16))] struct SliceArcInnerForStatic { inner : ArcInner < [u8 ; 1] > , }
};
}
