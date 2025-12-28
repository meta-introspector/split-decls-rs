macro_rules! FormatUnusedArg {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_format_unused_arg)] pub (crate) struct FormatUnusedArg { # [primary_span] # [label (builtin_macros_format_unused_arg)] pub (crate) span : Span , pub (crate) named : bool , }
    };
}

FormatUnusedArg!()