// Generated macro for rdseed32 (function)
macro_rules! Depcrate_randomrdseed32 {
() => {
// Module: crate::random
// Provides: {"rdseed32"}
// Dependencies: {}
# [doc = " Generates a 32-bit random value and stores it in `e`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] # [inline (always)] pub unsafe fn rdseed32 (e : & mut u32) -> bool { _rdseed32_step (e) == 1 }
};
}
