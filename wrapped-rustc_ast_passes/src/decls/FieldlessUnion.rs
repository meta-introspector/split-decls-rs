macro_rules! FieldlessUnion {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fieldless_union)] pub (crate) struct FieldlessUnion { # [primary_span] pub span : Span , }
    };
}

FieldlessUnion!()