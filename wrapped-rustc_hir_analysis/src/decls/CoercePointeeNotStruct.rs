macro_rules! CoercePointeeNotStruct {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_coerce_pointee_not_struct , code = E0802)] pub (crate) struct CoercePointeeNotStruct { # [primary_span] pub span : Span , pub kind : String , }
    };
}

CoercePointeeNotStruct!()