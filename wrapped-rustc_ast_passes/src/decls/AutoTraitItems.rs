macro_rules! AutoTraitItems {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_auto_items , code = E0380)] pub (crate) struct AutoTraitItems { # [primary_span] pub spans : Vec < Span > , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub total : Span , # [label] pub ident : Span , }
    };
}

AutoTraitItems!()