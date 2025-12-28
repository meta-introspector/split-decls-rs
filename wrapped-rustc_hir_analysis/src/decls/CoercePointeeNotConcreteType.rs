macro_rules! CoercePointeeNotConcreteType {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_coerce_pointee_not_concrete_ty , code = E0802)] pub (crate) struct CoercePointeeNotConcreteType { # [primary_span] pub span : Span , }
    };
}

CoercePointeeNotConcreteType!()