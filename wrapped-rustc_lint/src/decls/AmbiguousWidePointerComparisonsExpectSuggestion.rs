macro_rules! AmbiguousWidePointerComparisonsExpectSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_expect_suggestion , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousWidePointerComparisonsExpectSuggestion < 'a > { pub (crate) paren_left : & 'a str , pub (crate) paren_right : & 'a str , # [suggestion_part (code = r#"{{ #[expect(ambiguous_wide_pointer_comparisons, reason = "...")] {paren_left}"#)] pub (crate) before : Span , # [suggestion_part (code = "{paren_right} }}")] pub (crate) after : Span , }
    };
}

AmbiguousWidePointerComparisonsExpectSuggestion!()