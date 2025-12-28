macro_rules! TooLargeStatic {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_too_large_static)] pub (crate) struct TooLargeStatic { # [primary_span] pub span : Span , }
    };
}

TooLargeStatic!();