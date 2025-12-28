macro_rules! EmptySpecialization {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_empty_specialization)] pub (crate) struct EmptySpecialization { # [primary_span] pub span : Span , # [note] pub base_impl_span : Span , }
    };
}

EmptySpecialization!();