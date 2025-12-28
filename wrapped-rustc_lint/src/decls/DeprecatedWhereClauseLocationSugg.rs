macro_rules! DeprecatedWhereClauseLocationSugg {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum DeprecatedWhereClauseLocationSugg { # [multipart_suggestion (lint_suggestion_move_to_end , applicability = "machine-applicable")] MoveToEnd { # [suggestion_part (code = "")] left : Span , # [suggestion_part (code = "{sugg}")] right : Span , sugg : String , } , # [suggestion (lint_suggestion_remove_where , code = "" , applicability = "machine-applicable")] RemoveWhere { # [primary_span] span : Span , } , }
    };
}

DeprecatedWhereClauseLocationSugg!();