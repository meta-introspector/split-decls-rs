macro_rules! AmbiguousNegativeLiteralsNegativeLiteralSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_negative_literal , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousNegativeLiteralsNegativeLiteralSuggestion { # [suggestion_part (code = "(")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
    };
}

AmbiguousNegativeLiteralsNegativeLiteralSuggestion!();