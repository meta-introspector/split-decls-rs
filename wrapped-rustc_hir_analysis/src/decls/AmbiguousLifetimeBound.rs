macro_rules! AmbiguousLifetimeBound {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_ambiguous_lifetime_bound , code = E0227)] pub (crate) struct AmbiguousLifetimeBound { # [primary_span] pub span : Span , }
    };
}

AmbiguousLifetimeBound!();