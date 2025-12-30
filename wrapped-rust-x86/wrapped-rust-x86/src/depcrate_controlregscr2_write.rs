// Generated macro for cr2_write (function)
macro_rules! Depcrate_controlregscr2_write {
() => {
// Module: crate::controlregs
// Provides: {"cr2_write"}
// Dependencies: {}
# [doc = " Write cr2, for instance to reset cr2"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn cr2_write (val : u64) { asm ! ("mov {0}, %cr2" , in (reg) val as usize , options (att_syntax)) ; }
};
}
