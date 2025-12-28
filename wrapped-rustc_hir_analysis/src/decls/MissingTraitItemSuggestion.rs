macro_rules! MissingTraitItemSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (hir_analysis_missing_trait_item_suggestion , style = "tool-only" , applicability = "has-placeholders" , code = "{code}")] pub (crate) struct MissingTraitItemSuggestion { # [primary_span] pub span : Span , pub code : String , pub snippet : String , }
    };
}

MissingTraitItemSuggestion!()