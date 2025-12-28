macro_rules! InvalidSafetyOnFnPtr {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fn_ptr_invalid_safety)] pub (crate) struct InvalidSafetyOnFnPtr { # [primary_span] pub span : Span , }
    };
}

InvalidSafetyOnFnPtr!()