// Generated macro for macro_145 (macro)
macro_rules! Depcrate_bits32_pagingmacro_145 {
() => {
// Module: crate::bits32::paging
// Provides: {"macro_145"}
// Dependencies: {}
bitflags ! { # [doc = " PT Entry bits description."] # [repr (transparent)] pub struct PTFlags : u32 { # [doc = " Present; must be 1 to map a 4-KByte page."] const P = bit ! (0) ; # [doc = " Read/write; if 0, writes may not be allowed to the 4-KByte page referenced by this entry."] const RW = bit ! (1) ; # [doc = " User/supervisor; if 0, user-mode accesses are not allowed to the 4-KByte page referenced by this entry."] const US = bit ! (2) ; # [doc = " Page-level write-through."] const PWT = bit ! (3) ; # [doc = " Page-level cache disable."] const PCD = bit ! (4) ; # [doc = " Accessed; indicates whether software has accessed the 4-KByte page referenced by this entry."] const A = bit ! (5) ; # [doc = " Dirty; indicates whether software has written to the 4-KByte page referenced by this entry."] const D = bit ! (6) ; # [doc = " If the PAT is supported, indirectly determines the memory type used to access the 4-KByte page referenced by this entry;"] # [doc = " otherwise, reserved (must be 0)"] const PAT = bit ! (7) ; # [doc = " Global; if CR4.PGE = 1, determines whether the translation is global; ignored otherwise."] const G = bit ! (8) ; } }
};
}
