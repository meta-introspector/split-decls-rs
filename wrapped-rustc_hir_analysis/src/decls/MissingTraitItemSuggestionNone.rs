macro_rules! MissingTraitItemSuggestionNone {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (hir_analysis_missing_trait_item_suggestion , style = "hidden" , applicability = "has-placeholders" , code = "{code}")] pub (crate) struct MissingTraitItemSuggestionNone { # [primary_span] pub span : Span , pub code : String , pub snippet : String , }
    };
}

MissingTraitItemSuggestionNone!()