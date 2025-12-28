macro_rules! AddReturnTypeSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum AddReturnTypeSuggestion { # [suggestion (hir_typeck_add_return_type_add , code = " -> {found}" , applicability = "machine-applicable")] Add { # [primary_span] span : Span , found : String , } , # [suggestion (hir_typeck_add_return_type_missing_here , code = " -> _" , applicability = "has-placeholders")] MissingHere { # [primary_span] span : Span , } , }
    };
}

AddReturnTypeSuggestion!()