macro_rules! TestsNotSupport {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_tests_not_support)] pub (crate) struct TestsNotSupport { }
    };
}

TestsNotSupport!()