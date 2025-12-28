macro_rules! AssocItemIsPrivate {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_assoc_item_is_private , code = E0624)] pub (crate) struct AssocItemIsPrivate { # [primary_span] # [label] pub span : Span , pub kind : & 'static str , pub name : Ident , # [label (hir_analysis_defined_here_label)] pub defined_here_label : Span , }
    };
}

AssocItemIsPrivate!();