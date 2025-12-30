// Generated macro for TestAssertionFailure (struct)
macro_rules! Depcrate_internal_test_outcomeTestAssertionFailure {
() => {
// Module: crate::internal::test_outcome
// Provides: {"TestAssertionFailure"}
// Dependencies: {}
# [doc = " A report that a single test assertion failed."] # [doc = ""] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [doc = ""] # [doc = " See pub type Result in googletest/src/lib.rs for the public alias."] # [doc (hidden)] # [derive (Clone)] pub struct TestAssertionFailure { # [doc = " A human-readable formatted string describing the error."] pub description : String , pub custom_message : Option < String > , location : Location , }
};
}
