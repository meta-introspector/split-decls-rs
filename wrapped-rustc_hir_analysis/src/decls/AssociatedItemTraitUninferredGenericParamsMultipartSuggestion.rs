macro_rules! AssociatedItemTraitUninferredGenericParamsMultipartSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_analysis_associated_type_trait_uninferred_generic_params_multipart_suggestion , applicability = "maybe-incorrect")] pub (crate) struct AssociatedItemTraitUninferredGenericParamsMultipartSuggestion { # [suggestion_part (code = "{first}")] pub fspan : Span , pub first : String , # [suggestion_part (code = "{second}")] pub sspan : Span , pub second : String , }
    };
}

AssociatedItemTraitUninferredGenericParamsMultipartSuggestion!();