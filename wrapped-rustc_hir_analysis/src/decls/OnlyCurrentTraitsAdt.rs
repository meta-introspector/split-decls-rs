macro_rules! OnlyCurrentTraitsAdt {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_only_current_traits_adt)] pub (crate) struct OnlyCurrentTraitsAdt { # [primary_span] pub span : Span , pub name : String , }
    };
}

OnlyCurrentTraitsAdt!();