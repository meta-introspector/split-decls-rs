// Generated macro for cr3_write (function)
macro_rules! Depcrate_controlregscr3_write {
() => {
// Module: crate::controlregs
// Provides: {"cr3_write"}
// Dependencies: {}
# [doc = " Switch page-table PML4 pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr3_write (val : u64) { asm ! ("mov {0}, %cr3" , in (reg) val as usize , options (att_syntax)) ; }
};
}
