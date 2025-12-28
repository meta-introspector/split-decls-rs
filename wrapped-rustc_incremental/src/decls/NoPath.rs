macro_rules! NoPath {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_no_path)] pub (crate) struct NoPath { # [primary_span] pub span : Span , pub target : Symbol , pub source : String , }
    };
}

NoPath!()