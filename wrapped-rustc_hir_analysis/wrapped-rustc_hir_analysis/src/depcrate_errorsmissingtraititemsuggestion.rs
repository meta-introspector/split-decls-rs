// Generated macro for MissingTraitItemSuggestion (struct)
macro_rules! Depcrate_errorsMissingTraitItemSuggestion {
() => {
// Module: crate::errors
// Provides: {"MissingTraitItemSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (hir_analysis_missing_trait_item_suggestion , style = "tool-only" , applicability = "has-placeholders" , code = "{code}")] pub (crate) struct MissingTraitItemSuggestion { # [primary_span] pub span : Span , pub code : String , pub snippet : String , }
};
}
