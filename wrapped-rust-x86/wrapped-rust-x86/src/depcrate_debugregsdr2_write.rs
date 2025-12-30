// Generated macro for dr2_write (function)
macro_rules! Depcrate_debugregsdr2_write {
() => {
// Module: crate::debugregs
// Provides: {"dr2_write"}
// Dependencies: {}
# [doc = " Write dr2."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr2_write (val : usize) { asm ! ("mov {}, %dr2" , in (reg) val , options (att_syntax)) ; }
};
}
