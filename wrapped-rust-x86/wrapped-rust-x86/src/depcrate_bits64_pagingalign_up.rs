// Generated macro for align_up (function)
macro_rules! Depcrate_bits64_pagingalign_up {
() => {
// Module: crate::bits64::paging
// Provides: {"align_up"}
// Dependencies: {}
# [doc = " Align address upwards."] # [doc = ""] # [doc = " Returns the smallest x with alignment `align` so that x >= addr."] # [doc = " The alignment must be a power of 2."] # [inline (always)] fn align_up (addr : u64 , align : u64) -> u64 { let align_mask = align - 1 ; if addr & align_mask == 0 { addr } else { (addr | align_mask) + 1 } }
};
}
