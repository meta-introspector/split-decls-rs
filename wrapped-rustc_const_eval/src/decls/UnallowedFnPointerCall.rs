macro_rules! UnallowedFnPointerCall {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unallowed_fn_pointer_call)] pub (crate) struct UnallowedFnPointerCall { # [primary_span] pub span : Span , pub kind : ConstContext , }
    };
}

UnallowedFnPointerCall!();