macro_rules! FnParamForbiddenAttr {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fn_param_forbidden_attr)] pub (crate) struct FnParamForbiddenAttr { # [primary_span] pub span : Span , }
    };
}

FnParamForbiddenAttr!();