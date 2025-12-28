macro_rules! ConstParamTyImplOnUnsized {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_const_param_ty_impl_on_unsized)] pub (crate) struct ConstParamTyImplOnUnsized { # [primary_span] # [label] pub span : Span , }
    };
}

ConstParamTyImplOnUnsized!();