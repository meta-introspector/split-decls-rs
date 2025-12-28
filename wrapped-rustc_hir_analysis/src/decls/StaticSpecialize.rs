macro_rules! StaticSpecialize {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_static_specialize)] pub (crate) struct StaticSpecialize { # [primary_span] pub span : Span , }
    };
}

StaticSpecialize!();