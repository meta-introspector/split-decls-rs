macro_rules! FieldMultiplySpecifiedInInitializer {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_field_multiply_specified_in_initializer , code = E0062)] pub (crate) struct FieldMultiplySpecifiedInInitializer { # [primary_span] # [label] pub span : Span , # [label (hir_typeck_previous_use_label)] pub prev_span : Span , pub ident : Ident , }
    };
}

FieldMultiplySpecifiedInInitializer!()