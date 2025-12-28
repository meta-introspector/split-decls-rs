macro_rules! AsyncBoundOnlyForFnTraits {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_async_bound_only_for_fn_traits)] pub (crate) struct AsyncBoundOnlyForFnTraits { # [primary_span] pub span : Span , }
    };
}

AsyncBoundOnlyForFnTraits!()