macro_rules! UndroppedManuallyDropsSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "machine-applicable")] pub (crate) struct UndroppedManuallyDropsSuggestion { # [suggestion_part (code = "std::mem::ManuallyDrop::into_inner(")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
    };
}

UndroppedManuallyDropsSuggestion!()