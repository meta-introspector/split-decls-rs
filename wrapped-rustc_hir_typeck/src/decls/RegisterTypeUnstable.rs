macro_rules! RegisterTypeUnstable {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_register_type_unstable)] pub (crate) struct RegisterTypeUnstable < 'a > { # [primary_span] pub span : Span , pub ty : Ty < 'a > , }
    };
}

RegisterTypeUnstable!()