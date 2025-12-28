macro_rules! MissingIfThisChanged {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_missing_if_this_changed)] pub (crate) struct MissingIfThisChanged { # [primary_span] pub span : Span , }
    };
}

MissingIfThisChanged!();