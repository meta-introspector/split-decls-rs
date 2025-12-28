macro_rules! UselessImplItem {
    () => {
        # [derive (LintDiagnostic)] # [diag (hir_analysis_useless_impl_item)] pub (crate) struct UselessImplItem ;
    };
}

UselessImplItem!()