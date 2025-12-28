macro_rules! AsyncClosureSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "maybe-incorrect")] struct AsyncClosureSugg { # [suggestion_part (code = "")] deletion_span : Span , # [suggestion_part (code = "async ")] insertion_span : Span , }
    };
}

AsyncClosureSugg!()