// Generated macro for TraitFnConst (struct)
macro_rules! Depcrate_errorsTraitFnConst {
() => {
// Module: crate::errors
// Provides: {"TraitFnConst"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_trait_fn_const , code = E0379)] pub (crate) struct TraitFnConst { # [primary_span] # [label] pub span : Span , pub in_impl : bool , # [label (ast_passes_const_context_label)] pub const_context_label : Option < Span > , # [suggestion (ast_passes_remove_const_sugg , code = "")] pub remove_const_sugg : (Span , Applicability) , pub requires_multiple_changes : bool , # [suggestion (ast_passes_make_impl_const_sugg , code = "const " , applicability = "maybe-incorrect")] pub make_impl_const_sugg : Option < Span > , # [suggestion (ast_passes_make_trait_const_sugg , code = "#[const_trait]\n" , applicability = "maybe-incorrect")] pub make_trait_const_sugg : Option < Span > , }
};
}
