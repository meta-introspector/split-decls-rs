macro_rules! InvalidCallee {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_invalid_callee , code = E0618)] pub (crate) struct InvalidCallee < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub found : String , }
    };
}

InvalidCallee!();