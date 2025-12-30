// Generated macro for dr7_write (function)
macro_rules! Depcrate_debugregsdr7_write {
() => {
// Module: crate::debugregs
// Provides: {"dr7_write"}
// Dependencies: {}
# [doc = " Write dr7."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr7_write (val : Dr7) { asm ! ("mov {}, %dr7" , in (reg) val . 0 , options (att_syntax)) ; }
};
}
