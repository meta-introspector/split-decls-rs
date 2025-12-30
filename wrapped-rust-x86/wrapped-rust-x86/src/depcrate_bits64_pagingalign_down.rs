// Generated macro for align_down (function)
macro_rules! Depcrate_bits64_pagingalign_down {
() => {
// Module: crate::bits64::paging
// Provides: {"align_down"}
// Dependencies: {}
# [doc = " Align address downwards."] # [doc = ""] # [doc = " Returns the greatest x with alignment `align` so that x <= addr."] # [doc = " The alignment must be a power of 2."] # [inline (always)] fn align_down (addr : u64 , align : u64) -> u64 { addr & ! (align - 1) }
};
}
