// Generated macro for load_ldtr (function)
macro_rules! Depcrate_dtablesload_ldtr {
() => {
// Module: crate::dtables
// Provides: {"load_ldtr"}
// Dependencies: {}
# [doc = " Loads the segment selector into the selector field of the local"] # [doc = " descriptor table register (LDTR)."] # [doc = ""] # [doc = " After the segment selector is loaded in the LDTR,"] # [doc = " the processor uses the segment selector to locate"] # [doc = " the segment descriptor for the LDT in the global"] # [doc = " descriptor table (GDT)."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_ldtr (selector : SegmentSelector) { asm ! ("lldt {0:x}" , in (reg) selector . bits () , options (att_syntax)) ; }
};
}
