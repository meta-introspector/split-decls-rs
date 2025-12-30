// Generated macro for layout_to_flags (function)
macro_rules! Depcratelayout_to_flags {
() => {
// Module: crate
// Provides: {"layout_to_flags"}
// Dependencies: {}
# [doc = " If `align` is less than `_Alignof(max_align_t)`, and if the requested"] # [doc = " allocation `size` is larger than the alignment, we are guaranteed to get a"] # [doc = " suitably aligned allocation by default, without passing extra flags, and"] # [doc = " this function returns `0`."] # [doc = ""] # [doc = " Otherwise, it returns the alignment flag to pass to the jemalloc APIs."] fn layout_to_flags (align : usize , size : usize) -> c_int { if align <= ALIGNOF_MAX_ALIGN_T && align <= size { 0 } else { ffi :: MALLOCX_ALIGN (align) } }
};
}
