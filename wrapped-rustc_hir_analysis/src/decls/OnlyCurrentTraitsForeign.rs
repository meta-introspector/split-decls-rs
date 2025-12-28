macro_rules! OnlyCurrentTraitsForeign {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_only_current_traits_foreign)] pub (crate) struct OnlyCurrentTraitsForeign { # [primary_span] pub span : Span , }
    };
}

OnlyCurrentTraitsForeign!()