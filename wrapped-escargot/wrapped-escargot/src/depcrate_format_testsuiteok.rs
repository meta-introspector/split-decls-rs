// Generated macro for SuiteOk (struct)
macro_rules! Depcrate_format_testSuiteOk {
() => {
// Module: crate::format::test
// Provides: {"SuiteOk"}
// Dependencies: {}
# [doc = " Suite-finished successfully event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct SuiteOk { # [doc = " Cases that passed."] pub passed : usize , # [doc = " Cases that failed."] pub failed : usize , # [doc = " Cases that were allowed to fail."] pub allowed_fail : usize , # [doc = " Ignored cases."] pub ignored : usize , # [doc = " Benchmarks"] pub measured : usize , # [doc = " Cases filtered out by caller."] pub filtered_out : usize , }
};
}
