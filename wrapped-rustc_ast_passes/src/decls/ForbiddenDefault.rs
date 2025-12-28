macro_rules! ForbiddenDefault {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_forbidden_default)] pub (crate) struct ForbiddenDefault { # [primary_span] pub span : Span , # [label] pub def_span : Span , }
    };
}

ForbiddenDefault!();