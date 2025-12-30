// Generated macro for macro_313 (macro)
macro_rules! Depcrate_bits64_pagingmacro_313 {
() => {
// Module: crate::bits64::paging
// Provides: {"macro_313"}
// Dependencies: {}
bitflags ! { # [doc = " PT Entry bits description."] # [repr (transparent)] pub struct PTFlags : u64 { # [doc = " Present; must be 1 to map a 4-KByte page."] const P = bit ! (0) ; # [doc = " Read/write; if 0, writes may not be allowed to the 4-KByte region controlled by this entry"] const RW = bit ! (1) ; # [doc = " User/supervisor; user-mode accesses are not allowed to the 4-KByte region controlled by this entry."] const US = bit ! (2) ; # [doc = " Page-level write-through."] const PWT = bit ! (3) ; # [doc = " Page-level cache disable."] const PCD = bit ! (4) ; # [doc = " Accessed; indicates whether software has accessed the 4-KByte page"] const A = bit ! (5) ; # [doc = " Dirty; indicates whether software has written to the 4-KByte page referenced by this entry."] const D = bit ! (6) ; # [doc = " Global; if CR4.PGE = 1, determines whether the translation is global (see Section 4.10); ignored otherwise"] const G = bit ! (8) ; # [doc = " User defined flag -- ignored by hardware (bit 9)"] const USER_9 = bit ! (9) ; # [doc = " User defined flag -- ignored by hardware (bit 10)"] const USER_10 = bit ! (10) ; # [doc = " User defined flag -- ignored by hardware (bit 11)"] const USER_11 = bit ! (11) ; # [doc = " If IA32_EFER.NXE = 1, execute-disable"] # [doc = " If 1, instruction fetches are not allowed from the 512-GByte region."] const XD = bit ! (63) ; } }
};
}
