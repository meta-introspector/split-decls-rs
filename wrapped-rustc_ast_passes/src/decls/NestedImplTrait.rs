macro_rules! NestedImplTrait {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_nested_impl_trait , code = E0666)] pub (crate) struct NestedImplTrait { # [primary_span] pub span : Span , # [label (ast_passes_outer)] pub outer : Span , # [label (ast_passes_inner)] pub inner : Span , }
    };
}

NestedImplTrait!();