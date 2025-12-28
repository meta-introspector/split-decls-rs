macro_rules! AutoTraitBounds {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_auto_super_lifetime , code = E0568)] pub (crate) struct AutoTraitBounds { # [primary_span] pub span : Vec < Span > , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub removal : Span , # [label] pub ident : Span , }
    };
}

AutoTraitBounds!()