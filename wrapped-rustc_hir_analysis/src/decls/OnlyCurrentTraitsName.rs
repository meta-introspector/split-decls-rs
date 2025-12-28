macro_rules! OnlyCurrentTraitsName {
    () => {
        # [derive (Subdiagnostic)] # [label (hir_analysis_only_current_traits_name)] pub (crate) struct OnlyCurrentTraitsName < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }
    };
}

OnlyCurrentTraitsName!()