// Generated macro for TraitObjectDeclaredWithNoTraits (struct)
macro_rules! Depcrate_errorsTraitObjectDeclaredWithNoTraits {
() => {
// Module: crate::errors
// Provides: {"TraitObjectDeclaredWithNoTraits"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_trait_object_declared_with_no_traits , code = E0224)] pub (crate) struct TraitObjectDeclaredWithNoTraits { # [primary_span] pub span : Span , # [label (hir_analysis_alias_span)] pub trait_alias_span : Option < Span > , }
};
}
