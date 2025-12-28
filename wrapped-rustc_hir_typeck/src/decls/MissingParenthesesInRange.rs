macro_rules! deps {
    () => {
        AddMissingParenthesesInRange!();
    };
}

macro_rules! MissingParenthesesInRange {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_typeck_missing_parentheses_in_range , code = E0689)] pub (crate) struct MissingParenthesesInRange < 'tcx > { # [primary_span] # [label (hir_typeck_missing_parentheses_in_range)] pub span : Span , pub ty : Ty < 'tcx > , pub method_name : String , # [subdiagnostic] pub add_missing_parentheses : Option < AddMissingParenthesesInRange > , }
    };
}

MissingParenthesesInRange!();