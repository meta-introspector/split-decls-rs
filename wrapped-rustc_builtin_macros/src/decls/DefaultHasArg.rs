macro_rules! DefaultHasArg {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_default_arg)] pub (crate) struct DefaultHasArg { # [primary_span] # [suggestion (code = "#[default]" , style = "hidden" , applicability = "maybe-incorrect")] pub (crate) span : Span , }
    };
}

DefaultHasArg!();