// Generated macro for PML4_SLOT_SIZE (const)
macro_rules! Depcrate_bits64_pagingPML4_SLOT_SIZE {
() => {
// Module: crate::bits64::paging
// Provides: {"PML4_SLOT_SIZE"}
// Dependencies: {}
# [doc = " Size of a region covered by a PML4 Entry (512 GiB)"] # [cfg (target_arch = "x86_64")] pub const PML4_SLOT_SIZE : usize = HUGE_PAGE_SIZE * 512 ;
};
}
