macro_rules! InvalidSafetyOnExtern {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_extern_invalid_safety)] pub (crate) struct InvalidSafetyOnExtern { # [primary_span] pub item_span : Span , # [suggestion (code = "unsafe " , applicability = "machine-applicable" , style = "verbose")] pub block : Option < Span > , }
    };
}

InvalidSafetyOnExtern!()