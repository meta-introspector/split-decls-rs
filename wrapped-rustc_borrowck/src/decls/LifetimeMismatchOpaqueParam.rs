macro_rules! LifetimeMismatchOpaqueParam {
    () => {
        # [derive (Diagnostic)] # [diag (borrowck_opaque_type_lifetime_mismatch)] pub (crate) struct LifetimeMismatchOpaqueParam < 'tcx > { pub arg : GenericArg < 'tcx > , pub prev : GenericArg < 'tcx > , # [primary_span] # [label] # [note] pub span : Span , # [label (borrowck_prev_lifetime_label)] pub prev_span : Span , }
    };
}

LifetimeMismatchOpaqueParam!()