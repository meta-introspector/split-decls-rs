// Generated macro for pd_index (function)
macro_rules! Depcrate_bits64_pagingpd_index {
() => {
// Module: crate::bits64::paging
// Provides: {"pd_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PD."] # [inline] pub fn pd_index (addr : VAddr) -> usize { ((addr >> 21usize) & 0b111111111) as usize }
};
}
