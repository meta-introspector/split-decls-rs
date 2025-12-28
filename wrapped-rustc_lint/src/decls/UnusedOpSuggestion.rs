macro_rules! UnusedOpSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnusedOpSuggestion { # [suggestion (lint_suggestion , style = "verbose" , code = "let _ = " , applicability = "maybe-incorrect")] NormalExpr { # [primary_span] span : Span , } , # [multipart_suggestion (lint_suggestion , style = "verbose" , applicability = "maybe-incorrect")] BlockTailExpr { # [suggestion_part (code = "let _ = ")] before_span : Span , # [suggestion_part (code = ";")] after_span : Span , } , }
    };
}

UnusedOpSuggestion!();