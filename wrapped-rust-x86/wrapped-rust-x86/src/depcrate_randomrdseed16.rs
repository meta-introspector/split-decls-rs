// Generated macro for rdseed16 (function)
macro_rules! Depcrate_randomrdseed16 {
() => {
// Module: crate::random
// Provides: {"rdseed16"}
// Dependencies: {}
# [doc = " Generates a 16-bit random value and stores it in `e`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] # [inline (always)] pub unsafe fn rdseed16 (e : & mut u16) -> bool { _rdseed16_step (e) == 1 }
};
}
