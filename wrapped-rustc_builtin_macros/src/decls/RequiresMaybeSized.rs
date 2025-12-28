macro_rules! RequiresMaybeSized {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_maybe_sized , code = E0802)] struct RequiresMaybeSized { # [primary_span] span : Span , name : Ident , }
    };
}

RequiresMaybeSized!();