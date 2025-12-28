macro_rules! SuggestConvertViaMethod {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_convert_using_method , applicability = "machine-applicable" , style = "verbose")] pub (crate) struct SuggestConvertViaMethod < 'tcx > { # [suggestion_part (code = "{sugg}")] pub span : Span , # [suggestion_part (code = "")] pub borrow_removal_span : Option < Span > , pub sugg : String , pub expected : Ty < 'tcx > , pub found : Ty < 'tcx > , }
    };
}

SuggestConvertViaMethod!()