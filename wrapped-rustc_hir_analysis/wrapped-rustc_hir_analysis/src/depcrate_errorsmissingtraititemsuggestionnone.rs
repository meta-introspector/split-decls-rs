// Generated macro for MissingTraitItemSuggestionNone (struct)
macro_rules! Depcrate_errorsMissingTraitItemSuggestionNone {
() => {
// Module: crate::errors
// Provides: {"MissingTraitItemSuggestionNone"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (hir_analysis_missing_trait_item_suggestion , style = "hidden" , applicability = "has-placeholders" , code = "{code}")] pub (crate) struct MissingTraitItemSuggestionNone { # [primary_span] pub span : Span , pub code : String , pub snippet : String , }
};
}
