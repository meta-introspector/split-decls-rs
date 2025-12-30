// Generated macro for align_to (function)
macro_rules! Depcrate_alignmentalign_to {
() => {
// Module: crate::alignment
// Provides: {"align_to"}
// Dependencies: {}
# [doc = " Returns a multiple of `align` needed to store `size` bytes."] pub (crate) fn align_to (size : u64 , align : u64) -> u64 { (size + align - 1) & ! (align - 1) }
};
}
