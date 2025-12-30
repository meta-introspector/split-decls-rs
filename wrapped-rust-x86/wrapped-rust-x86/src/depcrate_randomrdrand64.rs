// Generated macro for rdrand64 (function)
macro_rules! Depcrate_randomrdrand64 {
() => {
// Module: crate::random
// Provides: {"rdrand64"}
// Dependencies: {}
# [doc = " Generates a 64-bit random value and stores it in `e`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] # [cfg (target_arch = "x86_64")] # [inline (always)] pub unsafe fn rdrand64 (e : & mut u64) -> bool { _rdrand64_step (e) == 1 }
};
}
