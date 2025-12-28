macro_rules! AddMissingParenthesesInRange {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_add_missing_parentheses_in_range , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AddMissingParenthesesInRange { pub func_name : String , # [suggestion_part (code = "(")] pub left : Span , # [suggestion_part (code = ")")] pub right : Span , }
    };
}

AddMissingParenthesesInRange!();