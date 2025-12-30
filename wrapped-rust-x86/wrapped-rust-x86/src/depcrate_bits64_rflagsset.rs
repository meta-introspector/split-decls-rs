// Generated macro for set (function)
macro_rules! Depcrate_bits64_rflagsset {
() => {
// Module: crate::bits64::rflags
// Provides: {"set"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] # [inline (always)] pub fn set (val : RFlags) { unsafe { asm ! ("pushq {0}; popfq" , in (reg) val . bits () , options (att_syntax)) ; } }
};
}
