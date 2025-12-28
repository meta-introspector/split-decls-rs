macro_rules! CoercePointeeNotTransparent {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_coerce_pointee_not_transparent , code = E0802)] pub (crate) struct CoercePointeeNotTransparent { # [primary_span] pub span : Span , }
    };
}

CoercePointeeNotTransparent!()