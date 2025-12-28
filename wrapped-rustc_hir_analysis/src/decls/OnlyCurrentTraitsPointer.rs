macro_rules! OnlyCurrentTraitsPointer {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_only_current_traits_pointer)] pub (crate) struct OnlyCurrentTraitsPointer < 'a > { # [primary_span] pub span : Span , pub pointer : Ty < 'a > , }
    };
}

OnlyCurrentTraitsPointer!();