// Generated macro for pt_index (function)
macro_rules! Depcrate_bits32_pagingpt_index {
() => {
// Module: crate::bits32::paging
// Provides: {"pt_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PT."] # [inline] pub fn pt_index (addr : VAddr) -> usize { ((addr >> 12usize) & 0b1111111111) as usize }
};
}
