macro_rules! deps {
    () => {
        InvalidUnionFieldSuggestion!();
    };
}

macro_rules! InvalidUnionField {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_invalid_union_field , code = E0740)] pub (crate) struct InvalidUnionField { # [primary_span] pub field_span : Span , # [subdiagnostic] pub sugg : InvalidUnionFieldSuggestion , # [note] pub note : () , }
    };
}

InvalidUnionField!()