// Generated macro for slice_from_ptr_range (function)
macro_rules! Depcrate_iterslice_from_ptr_range {
() => {
// Module: crate::iter
// Provides: {"slice_from_ptr_range"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Must ensure start and end point to the same memory object to uphold memory safety."] # [inline] unsafe fn slice_from_ptr_range < 'a > (start : * const u8 , end : * const u8) -> & 'a [u8] { debug_assert ! (start <= end) ; core :: slice :: from_raw_parts (start , end as usize - start as usize) }
};
}
