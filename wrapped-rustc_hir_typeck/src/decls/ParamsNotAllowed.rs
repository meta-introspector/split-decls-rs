macro_rules! ParamsNotAllowed {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_params_not_allowed)] # [help] pub (crate) struct ParamsNotAllowed { # [primary_span] pub span : Span , }
    };
}

ParamsNotAllowed!()