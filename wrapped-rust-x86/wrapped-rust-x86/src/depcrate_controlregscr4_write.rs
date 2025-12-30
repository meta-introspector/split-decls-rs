// Generated macro for cr4_write (function)
macro_rules! Depcrate_controlregscr4_write {
() => {
// Module: crate::controlregs
// Provides: {"cr4_write"}
// Dependencies: {}
# [doc = " Write cr4."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use x86::controlregs::*;"] # [doc = " unsafe {"] # [doc = "   let cr4 = cr4();"] # [doc = "   let cr4 = cr4 | Cr4::CR4_ENABLE_PSE;"] # [doc = "   cr4_write(cr4);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr4_write (val : Cr4) { asm ! ("mov {0}, %cr4" , in (reg) val . bits , options (att_syntax)) ; }
};
}
