// Generated macro for LIBSTDCXX_MIN_VERSION_THRESHOLD (const)
macro_rules! Depcrate_core_sanityLIBSTDCXX_MIN_VERSION_THRESHOLD {
() => {
// Module: crate::core::sanity
// Provides: {"LIBSTDCXX_MIN_VERSION_THRESHOLD"}
// Dependencies: {}
# [doc = " Minimum version threshold for libstdc++ required when using prebuilt LLVM"] # [doc = " from CI (with`llvm.download-ci-llvm` option)."] # [cfg (not (test))] const LIBSTDCXX_MIN_VERSION_THRESHOLD : usize = 8 ;
};
}
