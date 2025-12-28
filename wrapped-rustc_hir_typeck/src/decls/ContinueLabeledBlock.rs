macro_rules! ContinueLabeledBlock {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_continue_labeled_block , code = E0696)] pub (crate) struct ContinueLabeledBlock { # [primary_span] # [label] pub span : Span , # [label (hir_typeck_block_label)] pub block_span : Span , }
    };
}

ContinueLabeledBlock!()