macro_rules! NoFieldOnVariant {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_no_field_on_variant , code = E0609)] pub (crate) struct NoFieldOnVariant < 'tcx > { # [primary_span] pub (crate) span : Span , pub (crate) container : Ty < 'tcx > , pub (crate) ident : Ident , pub (crate) field : Ident , # [label (hir_typeck_no_field_on_variant_enum)] pub (crate) enum_span : Span , # [label (hir_typeck_no_field_on_variant_field)] pub (crate) field_span : Span , }
    };
}

NoFieldOnVariant!()