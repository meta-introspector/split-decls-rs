macro_rules! RequiresCfgPattern {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_requires_cfg_pattern)] pub (crate) struct RequiresCfgPattern { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

RequiresCfgPattern!()