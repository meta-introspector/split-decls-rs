// Generated macro for merge_coverage (function)
macro_rules! Depcratemerge_coverage {
() => {
// Module: crate
// Provides: {"merge_coverage"}
// Dependencies: {}
# [doc = " Merges previously dumped coverage data into the coverage counters."] # [doc = ""] # [doc = " This should be called prior to dumping if coverage data from a previous run"] # [doc = " already exists and should be merged with instead of overwritten."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is not thread-safe and should not be concurrently called from"] # [doc = " multiple threads."] pub unsafe fn merge_coverage (data : & [u8]) -> Result < () , IncompatibleCoverageData > { check_version () ; if __llvm_profile_check_compatibility (data . as_ptr () , data . len () as u64) == 0 && __llvm_profile_merge_from_buffer (data . as_ptr () , data . len () as u64) == 0 { Ok (()) } else { Err (IncompatibleCoverageData) } }
};
}
