// Generated macro for pml5_index (function)
macro_rules! Depcrate_bits64_pagingpml5_index {
() => {
// Module: crate::bits64::paging
// Provides: {"pml5_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PML5."] # [cfg (target_arch = "x86_64")] # [inline] pub fn pml5_index (addr : VAddr) -> usize { ((addr >> 48usize) & 0b111111111) as usize }
};
}
