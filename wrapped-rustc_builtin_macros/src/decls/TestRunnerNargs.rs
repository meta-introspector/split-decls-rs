macro_rules! TestRunnerNargs {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_test_runner_nargs)] pub (crate) struct TestRunnerNargs { # [primary_span] pub (crate) span : Span , }
    };
}

TestRunnerNargs!()