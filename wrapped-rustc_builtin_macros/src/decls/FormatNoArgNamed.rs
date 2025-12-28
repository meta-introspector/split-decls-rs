macro_rules! FormatNoArgNamed {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_format_no_arg_named)] # [note] # [note (builtin_macros_note2)] pub (crate) struct FormatNoArgNamed { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }
    };
}

FormatNoArgNamed!();