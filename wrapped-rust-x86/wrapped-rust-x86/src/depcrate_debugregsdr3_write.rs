// Generated macro for dr3_write (function)
macro_rules! Depcrate_debugregsdr3_write {
() => {
// Module: crate::debugregs
// Provides: {"dr3_write"}
// Dependencies: {}
# [doc = " Write dr3."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr3_write (val : usize) { asm ! ("mov {}, %dr3" , in (reg) val , options (att_syntax)) ; }
};
}
