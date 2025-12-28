macro_rules! HigherRankedSubtypeError {
    () => {
        # [derive (Diagnostic)] # [diag (borrowck_higher_ranked_subtype_error)] pub (crate) struct HigherRankedSubtypeError { # [primary_span] pub span : Span , }
    };
}

HigherRankedSubtypeError!()