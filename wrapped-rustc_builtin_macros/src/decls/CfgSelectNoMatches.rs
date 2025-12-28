macro_rules! CfgSelectNoMatches {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_cfg_select_no_matches)] pub (crate) struct CfgSelectNoMatches { # [primary_span] pub span : Span , }
    };
}

CfgSelectNoMatches!();