macro_rules! CfgAccessibleIndeterminate {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_cfg_accessible_indeterminate)] pub (crate) struct CfgAccessibleIndeterminate { # [primary_span] pub (crate) span : Span , }
    };
}

CfgAccessibleIndeterminate!()