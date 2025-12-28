macro_rules! NoPatterns {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_no_patterns)] pub (crate) struct NoPatterns { # [primary_span] pub span : Span , }
    };
}

NoPatterns!();