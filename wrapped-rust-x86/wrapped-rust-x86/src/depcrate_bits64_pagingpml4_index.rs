// Generated macro for pml4_index (function)
macro_rules! Depcrate_bits64_pagingpml4_index {
() => {
// Module: crate::bits64::paging
// Provides: {"pml4_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PML4."] # [cfg (target_arch = "x86_64")] # [inline] pub fn pml4_index (addr : VAddr) -> usize { ((addr >> 39usize) & 0b111111111) as usize }
};
}
