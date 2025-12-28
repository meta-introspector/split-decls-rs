macro_rules! RedundantSemicolonsSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_redundant_semicolons_suggestion , code = "" , applicability = "maybe-incorrect")] pub (crate) struct RedundantSemicolonsSuggestion { pub multiple_semicolons : bool , # [primary_span] pub span : Span , }
    };
}

RedundantSemicolonsSuggestion!();