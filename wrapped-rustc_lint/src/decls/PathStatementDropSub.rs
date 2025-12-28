macro_rules! PathStatementDropSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum PathStatementDropSub { # [suggestion (lint_suggestion , code = "drop({snippet});" , applicability = "machine-applicable")] Suggestion { # [primary_span] span : Span , snippet : String , } , # [help (lint_help)] Help { # [primary_span] span : Span , } , }
    };
}

PathStatementDropSub!()