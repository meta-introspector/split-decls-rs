macro_rules! SpecializationTrait {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_specialization_trait)] # [help] pub (crate) struct SpecializationTrait { # [primary_span] pub span : Span , }
    };
}

SpecializationTrait!();