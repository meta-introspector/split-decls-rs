// Generated macro for ldtr (function)
macro_rules! Depcrate_dtablesldtr {
() => {
// Module: crate::dtables
// Provides: {"ldtr"}
// Dependencies: {}
# [doc = " Returns the segment selector from the local descriptor table register (LDTR)."] # [doc = ""] # [doc = " The returned segment selector points to the segment descriptor"] # [doc = " (located in the GDT) for the current LDT."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn ldtr () -> SegmentSelector { let selector : u16 ; asm ! ("sldt {0:x}" , out (reg) selector , options (att_syntax)) ; SegmentSelector :: from_raw (selector) }
};
}
