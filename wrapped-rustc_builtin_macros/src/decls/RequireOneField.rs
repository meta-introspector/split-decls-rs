macro_rules! RequireOneField {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_field , code = E0802)] struct RequireOneField { # [primary_span] span : Span , }
    };
}

RequireOneField!()