macro_rules! CfgSelectUnreachable {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_cfg_select_unreachable)] pub (crate) struct CfgSelectUnreachable { # [primary_span] # [label (builtin_macros_label2)] pub span : Span , # [label] pub wildcard_span : Span , }
    };
}

CfgSelectUnreachable!()