macro_rules! TraceMacros {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_trace_macros)] pub (crate) struct TraceMacros { # [primary_span] pub (crate) span : Span , }
    };
}

TraceMacros!()