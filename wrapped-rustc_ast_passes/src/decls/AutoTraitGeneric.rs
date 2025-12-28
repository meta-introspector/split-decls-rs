macro_rules! AutoTraitGeneric {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_auto_generic , code = E0567)] pub (crate) struct AutoTraitGeneric { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub span : Span , # [label] pub ident : Span , }
    };
}

AutoTraitGeneric!();