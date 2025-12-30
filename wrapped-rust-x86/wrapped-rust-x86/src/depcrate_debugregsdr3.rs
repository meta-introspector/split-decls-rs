// Generated macro for dr3 (function)
macro_rules! Depcrate_debugregsdr3 {
() => {
// Module: crate::debugregs
// Provides: {"dr3"}
// Dependencies: {}
# [doc = " Read dr3."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr3 () -> usize { let ret : usize ; asm ! ("mov %dr3, {}" , out (reg) ret , options (att_syntax)) ; ret }
};
}
