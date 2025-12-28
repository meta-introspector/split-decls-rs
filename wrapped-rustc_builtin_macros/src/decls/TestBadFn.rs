macro_rules! TestBadFn {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_test_bad_fn)] pub (crate) struct TestBadFn { # [primary_span] pub (crate) span : Span , # [label] pub (crate) cause : Span , pub (crate) kind : & 'static str , }
    };
}

TestBadFn!();