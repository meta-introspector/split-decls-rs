macro_rules! ParenSugarAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_paren_sugar_attribute)] # [help] pub (crate) struct ParenSugarAttribute { # [primary_span] pub span : Span , }
    };
}

ParenSugarAttribute!();