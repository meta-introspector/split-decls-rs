macro_rules! ConstContinueBadLabel {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_const_continue_bad_label)] pub (crate) struct ConstContinueBadLabel { # [primary_span] pub span : Span , }
    };
}

ConstContinueBadLabel!()