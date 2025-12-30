// Generated macro for rdseed64 (function)
macro_rules! Depcrate_randomrdseed64 {
() => {
// Module: crate::random
// Provides: {"rdseed64"}
// Dependencies: {}
# [doc = " Generates a 64-bit random value and stores it in `e`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] # [cfg (target_arch = "x86_64")] # [inline (always)] pub unsafe fn rdseed64 (e : & mut u64) -> bool { _rdseed64_step (e) == 1 }
};
}
