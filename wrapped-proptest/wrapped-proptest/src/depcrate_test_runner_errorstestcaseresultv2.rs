// Generated macro for TestCaseResultV2 (type)
macro_rules! Depcrate_test_runner_errorsTestCaseResultV2 {
() => {
// Module: crate::test_runner::errors
// Provides: {"TestCaseResultV2"}
// Dependencies: {}
# [doc = " Intended to replace `TestCaseResult` in v2."] # [doc = ""] # [doc = " TODO-v2: Ideally `TestCaseResult = Result<TestCaseOk, TestCaseError>`"] # [doc = " however this breaks source compatibility in version 1.x.x because"] # [doc = " `TestCaseResult` is public."] pub (crate) type TestCaseResultV2 = Result < TestCaseOk , TestCaseError > ;
};
}
