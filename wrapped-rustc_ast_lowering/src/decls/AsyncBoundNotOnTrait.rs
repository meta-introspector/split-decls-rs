macro_rules! AsyncBoundNotOnTrait {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_async_bound_not_on_trait)] pub (crate) struct AsyncBoundNotOnTrait { # [primary_span] pub span : Span , pub descr : & 'static str , }
    };
}

AsyncBoundNotOnTrait!()