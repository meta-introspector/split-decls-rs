macro_rules! ItemUnderscore {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_item_underscore)] pub (crate) struct ItemUnderscore < 'a > { # [primary_span] # [label] pub span : Span , pub kind : & 'a str , }
    };
}

ItemUnderscore!();