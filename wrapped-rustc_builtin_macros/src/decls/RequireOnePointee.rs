macro_rules! RequireOnePointee {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_pointee , code = E0802)] struct RequireOnePointee { # [primary_span] span : Span , }
    };
}

RequireOnePointee!()