macro_rules! CopyImplOnTypeWithDtor {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_copy_impl_on_type_with_dtor , code = E0184)] pub (crate) struct CopyImplOnTypeWithDtor { # [primary_span] # [label] pub span : Span , }
    };
}

CopyImplOnTypeWithDtor!();