// Generated macro for TraitCannotImplForTy (struct)
macro_rules! Depcrate_errorsTraitCannotImplForTy {
() => {
// Module: crate::errors
// Provides: {"TraitCannotImplForTy"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_trait_cannot_impl_for_ty , code = E0204)] pub (crate) struct TraitCannotImplForTy { # [primary_span] pub span : Span , pub trait_name : String , # [label] pub label_spans : Vec < Span > , # [subdiagnostic] pub notes : Vec < ImplForTyRequires > , }
};
}
