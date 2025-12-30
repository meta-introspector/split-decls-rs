// Generated macro for cr0 (function)
macro_rules! Depcrate_controlregscr0 {
() => {
// Module: crate::controlregs
// Provides: {"cr0"}
// Dependencies: {}
# [doc = " Read cr0"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr0 () -> Cr0 { let ret : usize ; asm ! ("mov %cr0, {0}" , out (reg) ret , options (att_syntax)) ; Cr0 :: from_bits_truncate (ret) }
};
}
