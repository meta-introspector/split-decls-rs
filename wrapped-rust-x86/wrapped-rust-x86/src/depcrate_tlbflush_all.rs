// Generated macro for flush_all (function)
macro_rules! Depcrate_tlbflush_all {
() => {
// Module: crate::tlb
// Provides: {"flush_all"}
// Dependencies: {}
# [doc = " Invalidate the TLB completely by reloading the CR3 register."] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is unsafe as it causes a general protection fault (GP) if the current privilege"] # [doc = " level is not 0."] pub unsafe fn flush_all () { use crate :: controlregs :: { cr3 , cr3_write } ; cr3_write (cr3 ()) }
};
}
