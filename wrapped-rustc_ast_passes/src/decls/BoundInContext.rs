macro_rules! BoundInContext {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_bound_in_context)] pub (crate) struct BoundInContext < 'a > { # [primary_span] pub span : Span , pub ctx : & 'a str , }
    };
}

BoundInContext!()