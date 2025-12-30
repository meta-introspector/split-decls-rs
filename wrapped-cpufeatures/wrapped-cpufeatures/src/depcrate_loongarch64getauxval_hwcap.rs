// Generated macro for getauxval_hwcap (function)
macro_rules! Depcrate_loongarch64getauxval_hwcap {
() => {
// Module: crate::loongarch64
// Provides: {"getauxval_hwcap"}
// Dependencies: {}
# [doc = " Linux helper function for calling `getauxval` to get `AT_HWCAP`."] # [cfg (target_os = "linux")] pub fn getauxval_hwcap () -> u64 { unsafe { libc :: getauxval (libc :: AT_HWCAP) } }
};
}
