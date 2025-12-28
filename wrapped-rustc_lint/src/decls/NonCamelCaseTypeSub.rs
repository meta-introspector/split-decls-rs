macro_rules! NonCamelCaseTypeSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum NonCamelCaseTypeSub { # [label (lint_label)] Label { # [primary_span] span : Span , } , # [suggestion (lint_suggestion , code = "{replace}" , applicability = "maybe-incorrect")] Suggestion { # [primary_span] span : Span , replace : String , } , }
    };
}

NonCamelCaseTypeSub!();