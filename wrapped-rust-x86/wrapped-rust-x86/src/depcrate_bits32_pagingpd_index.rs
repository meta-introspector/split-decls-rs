// Generated macro for pd_index (function)
macro_rules! Depcrate_bits32_pagingpd_index {
() => {
// Module: crate::bits32::paging
// Provides: {"pd_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PD."] # [inline] pub fn pd_index (addr : VAddr) -> usize { ((addr >> 22usize) & 0b1111111111) as usize }
};
}
