macro_rules! AsyncFnInConstTraitOrTraitImpl {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_async_fn_in_const_trait_or_trait_impl)] pub (crate) struct AsyncFnInConstTraitOrTraitImpl { # [primary_span] pub async_keyword : Span , pub in_impl : bool , # [label] pub const_keyword : Span , }
    };
}

AsyncFnInConstTraitOrTraitImpl!()