macro_rules! RequireTransparent {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_transparent , code = E0802)] struct RequireTransparent { # [primary_span] span : Span , }
    };
}

RequireTransparent!();