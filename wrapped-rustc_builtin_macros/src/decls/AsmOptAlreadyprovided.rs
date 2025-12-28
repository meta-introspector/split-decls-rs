macro_rules! AsmOptAlreadyprovided {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_opt_already_provided)] pub (crate) struct AsmOptAlreadyprovided { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , }
    };
}

AsmOptAlreadyprovided!()