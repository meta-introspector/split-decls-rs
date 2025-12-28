macro_rules! NoFieldOnType {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_no_field_on_type , code = E0609)] pub (crate) struct NoFieldOnType < 'tcx > { # [primary_span] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) field : Ident , }
    };
}

NoFieldOnType!();