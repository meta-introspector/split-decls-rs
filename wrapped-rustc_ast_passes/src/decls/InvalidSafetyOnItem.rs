macro_rules! InvalidSafetyOnItem {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_item_invalid_safety)] pub (crate) struct InvalidSafetyOnItem { # [primary_span] pub span : Span , }
    };
}

InvalidSafetyOnItem!()