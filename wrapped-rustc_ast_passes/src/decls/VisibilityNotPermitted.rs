macro_rules! deps {
    () => {
        VisibilityNotPermittedNote!();
    };
}

macro_rules! VisibilityNotPermitted {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_visibility_not_permitted , code = E0449)] pub (crate) struct VisibilityNotPermitted { # [primary_span] pub span : Span , # [subdiagnostic] pub note : VisibilityNotPermittedNote , # [suggestion (ast_passes_remove_qualifier_sugg , code = "" , applicability = "machine-applicable")] pub remove_qualifier_sugg : Span , }
    };
}

VisibilityNotPermitted!();