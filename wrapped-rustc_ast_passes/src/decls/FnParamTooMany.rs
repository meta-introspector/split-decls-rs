macro_rules! FnParamTooMany {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_fn_param_too_many)] pub (crate) struct FnParamTooMany { # [primary_span] pub span : Span , pub max_num_args : usize , }
    };
}

FnParamTooMany!();