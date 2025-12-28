macro_rules! FieldAlreadyDeclaredNestedHelp {
    () => {
        # [derive (Subdiagnostic)] # [help (hir_analysis_field_already_declared_nested_help)] pub (crate) struct FieldAlreadyDeclaredNestedHelp { # [primary_span] pub span : Span , }
    };
}

FieldAlreadyDeclaredNestedHelp!();