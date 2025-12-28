macro_rules! OnlyCurrentTraitsOpaque {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_only_current_traits_opaque)] pub (crate) struct OnlyCurrentTraitsOpaque { # [primary_span] pub span : Span , }
    };
}

OnlyCurrentTraitsOpaque!();