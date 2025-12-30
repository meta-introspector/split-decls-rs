// Generated macro for dr0 (function)
macro_rules! Depcrate_debugregsdr0 {
() => {
// Module: crate::debugregs
// Provides: {"dr0"}
// Dependencies: {}
# [doc = " Read dr0."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr0 () -> usize { let ret : usize ; asm ! ("mov %dr0, {}" , out (reg) ret , options (att_syntax)) ; ret }
};
}
