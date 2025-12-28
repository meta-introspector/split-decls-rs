macro_rules! SlicingSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (hir_typeck_slicing_suggestion , code = "[..]" , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct SlicingSuggestion { # [primary_span] pub (crate) span : Span , }
    };
}

SlicingSuggestion!()