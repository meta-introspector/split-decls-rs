macro_rules! WhereClauseBeforeTypeAliasSugg {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum WhereClauseBeforeTypeAliasSugg { # [suggestion (ast_passes_remove_suggestion , applicability = "machine-applicable" , code = "")] Remove { # [primary_span] span : Span , } , # [multipart_suggestion (ast_passes_move_suggestion , applicability = "machine-applicable" , style = "verbose")] Move { # [suggestion_part (code = "")] left : Span , snippet : String , # [suggestion_part (code = "{snippet}")] right : Span , } , }
    };
}

WhereClauseBeforeTypeAliasSugg!()