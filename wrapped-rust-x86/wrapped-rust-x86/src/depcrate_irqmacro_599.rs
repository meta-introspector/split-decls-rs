// Generated macro for macro_599 (macro)
macro_rules! Depcrate_irqmacro_599 {
() => {
// Module: crate::irq
// Provides: {"macro_599"}
// Dependencies: {}
bitflags ! { pub struct PageFaultError : u32 { # [doc = " 0: The fault was caused by a non-present page."] # [doc = " 1: The fault was caused by a page-level protection violation"] const P = bit ! (0) ; # [doc = " 0: The access causing the fault was a read."] # [doc = " 1: The access causing the fault was a write."] const WR = bit ! (1) ; # [doc = " 0: The access causing the fault originated when the processor"] # [doc = " was executing in supervisor mode."] # [doc = " 1: The access causing the fault originated when the processor"] # [doc = " was executing in user mode."] const US = bit ! (2) ; # [doc = " 0: The fault was not caused by reserved bit violation."] # [doc = " 1: The fault was caused by reserved bits set to 1 in a page directory."] const RSVD = bit ! (3) ; # [doc = " 0: The fault was not caused by an instruction fetch."] # [doc = " 1: The fault was caused by an instruction fetch."] const ID = bit ! (4) ; # [doc = " 0: The fault was not by protection keys."] # [doc = " 1: There was a protection key violation."] const PK = bit ! (5) ; } }
};
}
