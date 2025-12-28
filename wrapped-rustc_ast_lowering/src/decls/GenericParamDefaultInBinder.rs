macro_rules! GenericParamDefaultInBinder {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_generic_param_default_in_binder)] pub (crate) struct GenericParamDefaultInBinder { # [primary_span] pub span : Span , }
    };
}

GenericParamDefaultInBinder!();