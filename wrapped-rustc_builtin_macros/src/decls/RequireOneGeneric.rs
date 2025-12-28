macro_rules! RequireOneGeneric {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_generic , code = E0802)] struct RequireOneGeneric { # [primary_span] span : Span , }
    };
}

RequireOneGeneric!()