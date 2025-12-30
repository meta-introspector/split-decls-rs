// Generated macro for rdrand16 (function)
macro_rules! Depcrate_randomrdrand16 {
() => {
// Module: crate::random
// Provides: {"rdrand16"}
// Dependencies: {}
# [doc = " Generates a 16-bit random value and stores it in `e`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] # [inline (always)] pub unsafe fn rdrand16 (e : & mut u16) -> bool { _rdrand16_step (e) == 1 }
};
}
