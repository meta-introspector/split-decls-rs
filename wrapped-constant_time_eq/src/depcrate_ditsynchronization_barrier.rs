// Generated macro for synchronization_barrier (function)
macro_rules! Depcrate_ditsynchronization_barrier {
() => {
// Module: crate::dit
// Provides: {"synchronization_barrier"}
// Dependencies: {}
# [doc = " Synchronization barrier for when `FEAT_SB` is not available."] # [doc = ""] # [doc = " This heavier alternative is recommended by Apple when SB is not available, see:"] # [doc = " <https://developer.apple.com/documentation/xcode/writing-arm64-code-for-apple-platforms>"] # [inline] fn synchronization_barrier () { unsafe { asm ! ("dsb nsh" , "isb sy" , options (nostack)) ; } }
};
}
