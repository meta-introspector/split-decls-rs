// Generated macro for cr2 (function)
macro_rules! Depcrate_controlregscr2 {
() => {
// Module: crate::controlregs
// Provides: {"cr2"}
// Dependencies: {}
# [doc = " Contains page-fault linear address."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr2 () -> usize { let ret : usize ; asm ! ("mov %cr2, {0}" , out (reg) ret , options (att_syntax)) ; ret }
};
}
