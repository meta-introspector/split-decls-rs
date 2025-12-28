macro_rules! FnParamCVarArgsNotLast {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fn_param_c_var_args_not_last)] pub (crate) struct FnParamCVarArgsNotLast { # [primary_span] pub span : Span , }
    };
}

FnParamCVarArgsNotLast!();