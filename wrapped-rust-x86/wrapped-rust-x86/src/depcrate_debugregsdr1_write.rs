// Generated macro for dr1_write (function)
macro_rules! Depcrate_debugregsdr1_write {
() => {
// Module: crate::debugregs
// Provides: {"dr1_write"}
// Dependencies: {}
# [doc = " Write dr1."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr1_write (val : usize) { asm ! ("mov {}, %dr1" , in (reg) val , options (att_syntax)) ; }
};
}
