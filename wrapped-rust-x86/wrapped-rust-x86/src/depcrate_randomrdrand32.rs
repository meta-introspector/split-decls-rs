// Generated macro for rdrand32 (function)
macro_rules! Depcrate_randomrdrand32 {
() => {
// Module: crate::random
// Provides: {"rdrand32"}
// Dependencies: {}
# [doc = " Generates a 32-bit random value and stores it in `e`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] # [inline (always)] pub unsafe fn rdrand32 (e : & mut u32) -> bool { _rdrand32_step (e) == 1 }
};
}
