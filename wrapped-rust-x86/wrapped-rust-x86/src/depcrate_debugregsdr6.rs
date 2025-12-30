// Generated macro for dr6 (function)
macro_rules! Depcrate_debugregsdr6 {
() => {
// Module: crate::debugregs
// Provides: {"dr6"}
// Dependencies: {}
# [doc = " Read dr6."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr6 () -> Dr6 { let ret : usize ; asm ! ("mov %dr6, {}" , out (reg) ret , options (att_syntax)) ; Dr6 :: from_bits_truncate (ret) }
};
}
