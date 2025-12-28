macro_rules! ExpectedItem {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_source_uitls_expected_item)] pub (crate) struct ExpectedItem < 'a > { # [primary_span] pub span : Span , pub token : & 'a str , }
    };
}

ExpectedItem!();