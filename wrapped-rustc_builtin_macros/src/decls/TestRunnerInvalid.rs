macro_rules! TestRunnerInvalid {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_test_runner_invalid)] pub (crate) struct TestRunnerInvalid { # [primary_span] pub (crate) span : Span , }
    };
}

TestRunnerInvalid!();