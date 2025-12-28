macro_rules! DerefImplsIsEmpty {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_typeck_deref_is_empty)] pub (crate) struct DerefImplsIsEmpty < 'tcx > { # [primary_span] pub span : Span , pub deref_ty : Ty < 'tcx > , }
    };
}

DerefImplsIsEmpty!()