// Generated macro for getauxval_hwcap (function)
macro_rules! Depcrate_aarch64getauxval_hwcap {
() => {
// Module: crate::aarch64
// Provides: {"getauxval_hwcap"}
// Dependencies: {}
# [doc = " Linux helper function for calling `getauxval` to get `AT_HWCAP`."] # [cfg (any (target_os = "linux" , target_os = "android"))] pub fn getauxval_hwcap () -> u64 { unsafe { libc :: getauxval (libc :: AT_HWCAP) } }
};
}
