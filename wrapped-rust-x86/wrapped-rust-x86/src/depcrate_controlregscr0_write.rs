// Generated macro for cr0_write (function)
macro_rules! Depcrate_controlregscr0_write {
() => {
// Module: crate::controlregs
// Provides: {"cr0_write"}
// Dependencies: {}
# [doc = " Write cr0."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr0_write (val : Cr0) { asm ! ("mov {0}, %cr0" , in (reg) val . bits , options (att_syntax)) ; }
};
}
