macro_rules! ForLoopsOverFalliblesSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "maybe-incorrect")] pub (crate) struct ForLoopsOverFalliblesSuggestion < 'a > { pub var : & 'a str , # [suggestion_part (code = "if let {var}(")] pub start_span : Span , # [suggestion_part (code = ") = ")] pub end_span : Span , }
    };
}

ForLoopsOverFalliblesSuggestion!()