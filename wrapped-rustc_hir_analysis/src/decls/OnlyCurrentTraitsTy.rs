macro_rules! OnlyCurrentTraitsTy {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_only_current_traits_ty)] pub (crate) struct OnlyCurrentTraitsTy < 'a > { # [primary_span] pub span : Span , pub ty : Ty < 'a > , }
    };
}

OnlyCurrentTraitsTy!();