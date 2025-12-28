macro_rules! LinkageType {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_linkage_type , code = E0791)] pub (crate) struct LinkageType { # [primary_span] pub span : Span , }
    };
}

LinkageType!()