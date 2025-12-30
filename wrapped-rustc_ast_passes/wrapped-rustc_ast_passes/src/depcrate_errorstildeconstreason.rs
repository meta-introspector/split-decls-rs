// Generated macro for TildeConstReason (enum)
macro_rules! Depcrate_errorsTildeConstReason {
() => {
// Module: crate::errors
// Provides: {"TildeConstReason"}
// Dependencies: {}
# [derive (Subdiagnostic , Copy , Clone)] pub (crate) enum TildeConstReason { # [note (ast_passes_closure)] Closure , # [note (ast_passes_function)] Function { # [primary_span] ident : Span , } , # [note (ast_passes_trait)] Trait { # [primary_span] span : Span , } , # [note (ast_passes_trait_impl)] TraitImpl { # [primary_span] span : Span , } , # [note (ast_passes_impl)] Impl { # [primary_span] span : Span , } , # [note (ast_passes_trait_assoc_ty)] TraitAssocTy { # [primary_span] span : Span , } , # [note (ast_passes_trait_impl_assoc_ty)] TraitImplAssocTy { # [primary_span] span : Span , } , # [note (ast_passes_inherent_assoc_ty)] InherentAssocTy { # [primary_span] span : Span , } , # [note (ast_passes_struct)] Struct { # [primary_span] span : Span , } , # [note (ast_passes_enum)] Enum { # [primary_span] span : Span , } , # [note (ast_passes_union)] Union { # [primary_span] span : Span , } , # [note (ast_passes_anon_const)] AnonConst { # [primary_span] span : Span , } , # [note (ast_passes_object)] TraitObject , # [note (ast_passes_item)] Item , }
};
}
