macro_rules! MissingUnsafeOnExtern {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_missing_unsafe_on_extern)] pub (crate) struct MissingUnsafeOnExtern { # [primary_span] pub span : Span , }
    };
}

MissingUnsafeOnExtern!()