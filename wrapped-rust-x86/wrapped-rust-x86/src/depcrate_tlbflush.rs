// Generated macro for flush (function)
macro_rules! Depcrate_tlbflush {
() => {
// Module: crate::tlb
// Provides: {"flush"}
// Dependencies: {}
# [doc = " Invalidate the given address in the TLB using the `invlpg` instruction."] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is unsafe as it causes a general protection fault (GP) if the current privilege"] # [doc = " level is not 0."] pub unsafe fn flush (addr : usize) { asm ! ("invlpg ({})" , in (reg) addr , options (att_syntax , nostack , preserves_flags)) ; }
};
}
