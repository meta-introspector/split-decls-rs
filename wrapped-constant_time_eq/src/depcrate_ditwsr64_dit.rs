// Generated macro for wsr64_dit (function)
macro_rules! Depcrate_ditwsr64_dit {
() => {
// Module: crate::dit
// Provides: {"wsr64_dit"}
// Dependencies: {}
# [doc = " Equivalent to `__arm_wsr64(\"dit\", value)`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Must be called only when `FEAT_DIT` is implemented."] # [inline] # [target_feature (enable = "dit")] unsafe fn wsr64_dit (value : u64) { unsafe { asm ! ("msr dit, {}" , in (reg) value , options (nostack)) ; } }
};
}
