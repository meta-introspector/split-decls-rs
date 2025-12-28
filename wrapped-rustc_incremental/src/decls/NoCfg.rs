macro_rules! NoCfg {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_no_cfg)] pub (crate) struct NoCfg { # [primary_span] pub span : Span , }
    };
}

NoCfg!();