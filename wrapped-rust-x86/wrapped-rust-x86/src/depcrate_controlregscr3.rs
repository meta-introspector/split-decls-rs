// Generated macro for cr3 (function)
macro_rules! Depcrate_controlregscr3 {
() => {
// Module: crate::controlregs
// Provides: {"cr3"}
// Dependencies: {}
# [doc = " Contains page-table root pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr3 () -> u64 { let ret : usize ; asm ! ("mov %cr3, {0}" , out (reg) ret , options (att_syntax)) ; ret as u64 }
};
}
