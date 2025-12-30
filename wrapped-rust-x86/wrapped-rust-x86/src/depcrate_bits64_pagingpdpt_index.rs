// Generated macro for pdpt_index (function)
macro_rules! Depcrate_bits64_pagingpdpt_index {
() => {
// Module: crate::bits64::paging
// Provides: {"pdpt_index"}
// Dependencies: {}
# [doc = " Given virtual address calculate corresponding entry in PDPT."] # [inline] pub fn pdpt_index (addr : VAddr) -> usize { ((addr >> 30usize) & 0b111111111) as usize }
};
}
