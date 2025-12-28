macro_rules! TestCaseNonItem {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_test_case_non_item)] pub (crate) struct TestCaseNonItem { # [primary_span] pub (crate) span : Span , }
    };
}

TestCaseNonItem!()