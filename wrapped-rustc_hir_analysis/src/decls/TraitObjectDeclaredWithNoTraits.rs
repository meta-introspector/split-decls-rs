macro_rules! TraitObjectDeclaredWithNoTraits {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_trait_object_declared_with_no_traits , code = E0224)] pub (crate) struct TraitObjectDeclaredWithNoTraits { # [primary_span] pub span : Span , # [label (hir_analysis_alias_span)] pub trait_alias_span : Option < Span > , }
    };
}

TraitObjectDeclaredWithNoTraits!()