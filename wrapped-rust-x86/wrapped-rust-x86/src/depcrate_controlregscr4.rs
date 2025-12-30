// Generated macro for cr4 (function)
macro_rules! Depcrate_controlregscr4 {
() => {
// Module: crate::controlregs
// Provides: {"cr4"}
// Dependencies: {}
# [doc = " Contains various flags to control operations in protected mode."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr4 () -> Cr4 { let ret : usize ; asm ! ("mov %cr4, {0}" , out (reg) ret , options (att_syntax)) ; Cr4 :: from_bits_truncate (ret) }
};
}
