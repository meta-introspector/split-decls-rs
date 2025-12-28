macro_rules! ExpectedOneCfgPattern {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_expected_one_cfg_pattern , code = E0536)] pub (crate) struct ExpectedOneCfgPattern { # [primary_span] pub span : Span , }
    };
}

ExpectedOneCfgPattern!();