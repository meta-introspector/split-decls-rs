macro_rules! CopyImplOnNonAdt {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_copy_impl_on_non_adt , code = E0206)] pub (crate) struct CopyImplOnNonAdt { # [primary_span] # [label] pub span : Span , }
    };
}

CopyImplOnNonAdt!();