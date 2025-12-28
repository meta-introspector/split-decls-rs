macro_rules! OneCfgPattern {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_expected_one_cfg_pattern)] pub (crate) struct OneCfgPattern { # [primary_span] pub (crate) span : Span , }
    };
}

OneCfgPattern!()