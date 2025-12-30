// Generated macro for data_offset (function)
macro_rules! Depcrate_syncdata_offset {
() => {
// Module: crate::sync
// Provides: {"data_offset"}
// Dependencies: {}
# [doc = " Gets the offset within an `ArcInner` for the payload behind a pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must point to (and have valid metadata for) a previously"] # [doc = " valid instance of T, but the T is allowed to be dropped."] unsafe fn data_offset < T : ? Sized > (ptr : * const T) -> usize { unsafe { data_offset_align (align_of_val_raw (ptr)) } }
};
}
