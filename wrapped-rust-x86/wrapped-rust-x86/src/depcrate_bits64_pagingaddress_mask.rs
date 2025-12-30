// Generated macro for ADDRESS_MASK (const)
macro_rules! Depcrate_bits64_pagingADDRESS_MASK {
() => {
// Module: crate::bits64::paging
// Provides: {"ADDRESS_MASK"}
// Dependencies: {}
# [doc = " Mask to find the physical address of an entry in a page-table."] const ADDRESS_MASK : u64 = ((1 << MAXPHYADDR) - 1) & ! 0xfff ;
};
}
