macro_rules! CaptureReasonSuggest {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum CaptureReasonSuggest < 'tcx > { # [suggestion (borrowck_suggest_iterate_over_slice , applicability = "maybe-incorrect" , code = "&" , style = "verbose")] IterateSlice { ty : Ty < 'tcx > , # [primary_span] span : Span , } , # [suggestion (borrowck_suggest_create_fresh_reborrow , applicability = "maybe-incorrect" , code = ".as_mut()" , style = "verbose")] FreshReborrow { # [primary_span] span : Span , } , }
    };
}

CaptureReasonSuggest!()