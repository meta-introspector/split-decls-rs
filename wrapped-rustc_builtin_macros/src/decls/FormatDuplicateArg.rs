macro_rules! FormatDuplicateArg {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_format_duplicate_arg)] pub (crate) struct FormatDuplicateArg { # [primary_span] pub (crate) span : Span , # [label (builtin_macros_label1)] pub (crate) prev : Span , # [label (builtin_macros_label2)] pub (crate) duplicate : Span , pub (crate) ident : Ident , }
    };
}

FormatDuplicateArg!()