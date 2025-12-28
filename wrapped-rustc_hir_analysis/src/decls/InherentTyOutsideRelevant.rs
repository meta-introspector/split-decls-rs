macro_rules! InherentTyOutsideRelevant {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_inherent_ty_outside_relevant , code = E0390)] # [help] pub (crate) struct InherentTyOutsideRelevant { # [primary_span] pub span : Span , # [help (hir_analysis_span_help)] pub help_span : Span , }
    };
}

InherentTyOutsideRelevant!()