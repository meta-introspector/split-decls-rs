// Generated macro for dr1 (function)
macro_rules! Depcrate_debugregsdr1 {
() => {
// Module: crate::debugregs
// Provides: {"dr1"}
// Dependencies: {}
# [doc = " Read dr1."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr1 () -> usize { let ret : usize ; asm ! ("mov %dr1, {}" , out (reg) ret , options (att_syntax)) ; ret }
};
}
