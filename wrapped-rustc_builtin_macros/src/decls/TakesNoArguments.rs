macro_rules! TakesNoArguments {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_takes_no_arguments)] pub (crate) struct TakesNoArguments < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }
    };
}

TakesNoArguments!();