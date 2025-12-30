// Generated macro for set (function)
macro_rules! Depcrate_bits32_eflagsset {
() => {
// Module: crate::bits32::eflags
// Provides: {"set"}
// Dependencies: {}
# [cfg (target_arch = "x86")] # [inline (always)] pub unsafe fn set (val : EFlags) { asm ! ("pushl {0}; popfl" , in (reg) val . bits () , options (att_syntax)) ; }
};
}
