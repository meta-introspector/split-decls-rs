// Generated macro for TestOutcome (enum)
macro_rules! Depcrate_internal_test_outcomeTestOutcome {
() => {
// Module: crate::internal::test_outcome
// Provides: {"TestOutcome"}
// Dependencies: {}
# [doc = " The outcome hitherto of running a test."] # [doc = ""] # [doc = " This is kept as a running record as the test progresses. One can access it"] # [doc = " with `TestOutcome::with_current_test_outcome`."] # [doc = ""] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [doc (hidden)] pub enum TestOutcome { # [doc = " The test ran or is currently running and no assertions have failed."] Success , # [doc = " The test ran or is currently running and at least one assertion has"] # [doc = " failed."] Failure , }
};
}
