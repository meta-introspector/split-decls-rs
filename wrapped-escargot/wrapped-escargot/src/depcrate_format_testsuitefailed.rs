// Generated macro for SuiteFailed (struct)
macro_rules! Depcrate_format_testSuiteFailed {
() => {
// Module: crate::format::test
// Provides: {"SuiteFailed"}
// Dependencies: {}
# [doc = " Suite-finished with failure event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct SuiteFailed { # [doc = " Cases that passed."] pub passed : usize , # [doc = " Cases that failed."] pub failed : usize , # [doc = " Cases that were allowed to fail."] pub allowed_fail : usize , # [doc = " Ignored cases."] pub ignored : usize , # [doc = " Benchmarks"] pub measured : usize , # [doc = " Cases filtered out by caller."] pub filtered_out : usize , }
};
}
