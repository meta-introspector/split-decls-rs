macro_rules! AsyncCoroutinesNotSupported {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_async_coroutines_not_supported , code = E0727)] pub (crate) struct AsyncCoroutinesNotSupported { # [primary_span] pub span : Span , }
    };
}

AsyncCoroutinesNotSupported!();