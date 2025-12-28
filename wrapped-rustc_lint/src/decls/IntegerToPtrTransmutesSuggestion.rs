macro_rules! IntegerToPtrTransmutesSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum IntegerToPtrTransmutesSuggestion < 'tcx > { # [multipart_suggestion (lint_suggestion_with_exposed_provenance , applicability = "machine-applicable" , style = "verbose")] ToPtr { dst : Ty < 'tcx > , suffix : & 'static str , # [suggestion_part (code = "std::ptr::with_exposed_provenance{suffix}::<{dst}>(")] start_call : Span , } , # [multipart_suggestion (lint_suggestion_with_exposed_provenance , applicability = "machine-applicable" , style = "verbose")] ToRef { dst : Ty < 'tcx > , suffix : & 'static str , ref_mutbl : & 'static str , # [suggestion_part (code = "&{ref_mutbl}*std::ptr::with_exposed_provenance{suffix}::<{dst}>(")] start_call : Span , } , }
    };
}

IntegerToPtrTransmutesSuggestion!()