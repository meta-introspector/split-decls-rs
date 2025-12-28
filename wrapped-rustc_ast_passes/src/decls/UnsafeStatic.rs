macro_rules! UnsafeStatic {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_unsafe_static)] pub (crate) struct UnsafeStatic { # [primary_span] pub span : Span , }
    };
}

UnsafeStatic!()