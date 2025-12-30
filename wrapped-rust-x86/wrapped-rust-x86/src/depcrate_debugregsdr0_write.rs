// Generated macro for dr0_write (function)
macro_rules! Depcrate_debugregsdr0_write {
() => {
// Module: crate::debugregs
// Provides: {"dr0_write"}
// Dependencies: {}
# [doc = " Write dr0."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr0_write (val : usize) { asm ! ("mov {}, %dr0" , in (reg) val , options (att_syntax)) ; }
};
}
