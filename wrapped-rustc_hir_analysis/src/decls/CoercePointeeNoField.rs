macro_rules! CoercePointeeNoField {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_coerce_pointee_no_field , code = E0802)] pub (crate) struct CoercePointeeNoField { # [primary_span] pub span : Span , }
    };
}

CoercePointeeNoField!();