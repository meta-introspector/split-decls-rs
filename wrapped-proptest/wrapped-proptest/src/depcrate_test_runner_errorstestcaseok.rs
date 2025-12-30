// Generated macro for TestCaseOk (enum)
macro_rules! Depcrate_test_runner_errorsTestCaseOk {
() => {
// Module: crate::test_runner::errors
// Provides: {"TestCaseOk"}
// Dependencies: {}
# [doc = " Indicates the type of test that ran successfully."] # [doc = ""] # [doc = " This is used for managing whether or not a success is counted against"] # [doc = " configured `PROPTEST_CASES`; only `NewCases` shall be counted."] # [doc = ""] # [doc = " TODO-v2: Ideally `TestCaseResult = Result<TestCaseOk, TestCaseError>`"] # [doc = " however this breaks source compatibility in version 1.x.x because"] # [doc = " `TestCaseResult` is public."] # [derive (Debug , Clone)] pub (crate) enum TestCaseOk { NewCaseSuccess , PersistedCaseSuccess , ReplayFromForkSuccess , CacheHitSuccess , Reject , }
};
}
