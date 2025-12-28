macro_rules! GenericDefaultTrailing {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_generic_default_trailing)] pub (crate) struct GenericDefaultTrailing { # [primary_span] pub span : Span , }
    };
}

GenericDefaultTrailing!();