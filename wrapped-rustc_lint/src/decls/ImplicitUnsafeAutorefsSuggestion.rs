macro_rules! ImplicitUnsafeAutorefsSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "maybe-incorrect")] pub (crate) struct ImplicitUnsafeAutorefsSuggestion { pub mutbl : & 'static str , pub deref : & 'static str , # [suggestion_part (code = "({mutbl}{deref}")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
    };
}

ImplicitUnsafeAutorefsSuggestion!();