macro_rules! deps {
    () => {
        FieldAlreadyDeclaredNestedHelp!();
    };
}

macro_rules! FieldAlreadyDeclared {
    () => {
        deps!();
        # [derive (Diagnostic)] pub (crate) enum FieldAlreadyDeclared { # [diag (hir_analysis_field_already_declared , code = E0124)] NotNested { field_name : Ident , # [primary_span] # [label] span : Span , # [label (hir_analysis_previous_decl_label)] prev_span : Span , } , # [diag (hir_analysis_field_already_declared_current_nested)] CurrentNested { field_name : Ident , # [primary_span] # [label] span : Span , # [note (hir_analysis_nested_field_decl_note)] nested_field_span : Span , # [subdiagnostic] help : FieldAlreadyDeclaredNestedHelp , # [label (hir_analysis_previous_decl_label)] prev_span : Span , } , # [diag (hir_analysis_field_already_declared_previous_nested)] PreviousNested { field_name : Ident , # [primary_span] # [label] span : Span , # [label (hir_analysis_previous_decl_label)] prev_span : Span , # [note (hir_analysis_previous_nested_field_decl_note)] prev_nested_field_span : Span , # [subdiagnostic] prev_help : FieldAlreadyDeclaredNestedHelp , } , # [diag (hir_analysis_field_already_declared_both_nested)] BothNested { field_name : Ident , # [primary_span] # [label] span : Span , # [note (hir_analysis_nested_field_decl_note)] nested_field_span : Span , # [subdiagnostic] help : FieldAlreadyDeclaredNestedHelp , # [label (hir_analysis_previous_decl_label)] prev_span : Span , # [note (hir_analysis_previous_nested_field_decl_note)] prev_nested_field_span : Span , # [subdiagnostic] prev_help : FieldAlreadyDeclaredNestedHelp , } , }
    };
}

FieldAlreadyDeclared!();