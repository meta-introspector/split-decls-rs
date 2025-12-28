macro_rules! AmbiguousNegativeLiteralsCurrentBehaviorSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_current_behavior , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousNegativeLiteralsCurrentBehaviorSuggestion { # [suggestion_part (code = "(")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
    };
}

AmbiguousNegativeLiteralsCurrentBehaviorSuggestion!()