// Generated macro for dr7 (function)
macro_rules! Depcrate_debugregsdr7 {
() => {
// Module: crate::debugregs
// Provides: {"dr7"}
// Dependencies: {}
# [doc = " Read dr7."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr7 () -> Dr7 { let ret : usize ; asm ! ("mov %dr7, {}" , out (reg) ret , options (att_syntax)) ; Dr7 (ret) }
};
}
