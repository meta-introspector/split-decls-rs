macro_rules! UnsafeItem {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_unsafe_item)] pub (crate) struct UnsafeItem { # [primary_span] pub span : Span , pub kind : & 'static str , }
    };
}

UnsafeItem!()