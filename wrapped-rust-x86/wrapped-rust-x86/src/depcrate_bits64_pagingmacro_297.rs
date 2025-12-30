// Generated macro for macro_297 (macro)
macro_rules! Depcrate_bits64_pagingmacro_297 {
() => {
// Module: crate::bits64::paging
// Provides: {"macro_297"}
// Dependencies: {}
bitflags ! { # [doc = " PML4 configuration bit description."] # [repr (transparent)] pub struct PML4Flags : u64 { # [doc = " Present; must be 1 to reference a page-directory-pointer table"] const P = bit ! (0) ; # [doc = " Read/write; if 0, writes may not be allowed to the 512-GByte region"] # [doc = " controlled by this entry (see Section 4.6)"] const RW = bit ! (1) ; # [doc = " User/supervisor; if 0, user-mode accesses are not allowed"] # [doc = " to the 512-GByte region controlled by this entry."] const US = bit ! (2) ; # [doc = " Page-level write-through; indirectly determines the memory type used to"] # [doc = " access the page-directory-pointer table referenced by this entry."] const PWT = bit ! (3) ; # [doc = " Page-level cache disable; indirectly determines the memory type used to"] # [doc = " access the page-directory-pointer table referenced by this entry."] const PCD = bit ! (4) ; # [doc = " Accessed; indicates whether this entry has been used for linear-address translation."] const A = bit ! (5) ; # [doc = " User defined flag -- ignored by hardware (bit 9)"] const USER_9 = bit ! (9) ; # [doc = " User defined flag -- ignored by hardware (bit 10)"] const USER_10 = bit ! (10) ; # [doc = " User defined flag -- ignored by hardware (bit 11)"] const USER_11 = bit ! (11) ; # [doc = " If IA32_EFER.NXE = 1, execute-disable"] # [doc = " If 1, instruction fetches are not allowed from the 512-GByte region."] const XD = bit ! (63) ; } }
};
}
