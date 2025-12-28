macro_rules! SuggestBoxing {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum SuggestBoxing { # [note (hir_typeck_suggest_boxing_note)] # [multipart_suggestion (hir_typeck_suggest_boxing_when_appropriate , applicability = "machine-applicable")] Unit { # [suggestion_part (code = "Box::new(())")] start : Span , # [suggestion_part (code = "")] end : Span , } , # [note (hir_typeck_suggest_boxing_note)] AsyncBody , # [note (hir_typeck_suggest_boxing_note)] # [multipart_suggestion (hir_typeck_suggest_boxing_when_appropriate , applicability = "machine-applicable")] ExprFieldShorthand { # [suggestion_part (code = "{ident}: Box::new(")] start : Span , # [suggestion_part (code = ")")] end : Span , ident : Ident , } , # [note (hir_typeck_suggest_boxing_note)] # [multipart_suggestion (hir_typeck_suggest_boxing_when_appropriate , applicability = "machine-applicable")] Other { # [suggestion_part (code = "Box::new(")] start : Span , # [suggestion_part (code = ")")] end : Span , } , }
    };
}

SuggestBoxing!()