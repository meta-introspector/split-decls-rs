macro_rules! ConstParamTyImplOnNonAdt {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_const_param_ty_impl_on_non_adt)] pub (crate) struct ConstParamTyImplOnNonAdt { # [primary_span] # [label] pub span : Span , }
    };
}

ConstParamTyImplOnNonAdt!()