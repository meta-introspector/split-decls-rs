macro_rules! NestedLifetimes {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_nested_lifetimes , code = E0316)] pub (crate) struct NestedLifetimes { # [primary_span] pub span : Span , }
    };
}

NestedLifetimes!();