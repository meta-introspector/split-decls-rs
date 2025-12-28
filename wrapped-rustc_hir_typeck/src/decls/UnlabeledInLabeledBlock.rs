macro_rules! UnlabeledInLabeledBlock {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_unlabeled_in_labeled_block , code = E0695)] pub (crate) struct UnlabeledInLabeledBlock < 'a > { # [primary_span] # [label] pub span : Span , pub cf_type : & 'a str , }
    };
}

UnlabeledInLabeledBlock!();