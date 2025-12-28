macro_rules! InherentTyOutside {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_inherent_ty_outside , code = E0390)] # [help] pub (crate) struct InherentTyOutside { # [primary_span] # [help (hir_analysis_span_help)] pub span : Span , }
    };
}

InherentTyOutside!()