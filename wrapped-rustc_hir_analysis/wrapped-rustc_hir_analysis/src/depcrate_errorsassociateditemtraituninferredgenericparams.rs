// Generated macro for AssociatedItemTraitUninferredGenericParams (struct)
macro_rules! Depcrate_errorsAssociatedItemTraitUninferredGenericParams {
() => {
// Module: crate::errors
// Provides: {"AssociatedItemTraitUninferredGenericParams"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_associated_type_trait_uninferred_generic_params , code = E0212)] pub (crate) struct AssociatedItemTraitUninferredGenericParams { # [primary_span] pub span : Span , # [suggestion (style = "verbose" , applicability = "maybe-incorrect" , code = "{bound}")] pub inferred_sugg : Option < Span > , pub bound : String , # [subdiagnostic] pub mpart_sugg : Option < AssociatedItemTraitUninferredGenericParamsMultipartSuggestion > , pub what : & 'static str , }
};
}
