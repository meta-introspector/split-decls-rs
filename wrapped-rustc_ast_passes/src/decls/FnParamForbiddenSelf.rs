macro_rules! FnParamForbiddenSelf {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fn_param_forbidden_self)] # [note] pub (crate) struct FnParamForbiddenSelf { # [primary_span] # [label] pub span : Span , }
    };
}

FnParamForbiddenSelf!()