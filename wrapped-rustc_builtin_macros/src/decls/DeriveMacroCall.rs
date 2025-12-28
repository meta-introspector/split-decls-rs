macro_rules! DeriveMacroCall {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_derive_macro_call)] pub (crate) struct DeriveMacroCall { # [primary_span] pub (crate) span : Span , }
    };
}

DeriveMacroCall!();