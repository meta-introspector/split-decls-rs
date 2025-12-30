// Generated macro for reset_coverage (function)
macro_rules! Depcratereset_coverage {
() => {
// Module: crate
// Provides: {"reset_coverage"}
// Dependencies: {}
# [doc = " Resets all coverage counters in the program to zero."] # [doc = ""] # [doc = " This function should be called after a process forks to avoid recording"] # [doc = " coverage data for the parent process twice."] # [doc = ""] # [doc = " You should also call this after calling `capture_coverage` if you intend to"] # [doc = " continue running with the intention of merging with the captured coverage"] # [doc = " later."] pub fn reset_coverage () { check_version () ; unsafe { __llvm_profile_reset_counters () ; } }
};
}
