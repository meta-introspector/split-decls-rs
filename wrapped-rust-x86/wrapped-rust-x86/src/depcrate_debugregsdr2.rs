// Generated macro for dr2 (function)
macro_rules! Depcrate_debugregsdr2 {
() => {
// Module: crate::debugregs
// Provides: {"dr2"}
// Dependencies: {}
# [doc = " Read dr2."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr2 () -> usize { let ret : usize ; asm ! ("mov %dr2, {}" , out (reg) ret , options (att_syntax)) ; ret }
};
}
