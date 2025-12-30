// Generated macro for pt_index (function)
macro_rules! Depcrate_bits64_pagingpt_index {
() => {
// Module: crate::bits64::paging
// Provides: {"pt_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PT."] # [inline] pub fn pt_index (addr : VAddr) -> usize { ((addr >> 12usize) & 0b111111111) as usize }
};
}
