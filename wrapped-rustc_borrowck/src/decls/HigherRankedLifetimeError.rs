macro_rules! deps {
    () => {
        HigherRankedErrorCause!();
    };
}

macro_rules! HigherRankedLifetimeError {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (borrowck_higher_ranked_lifetime_error)] pub (crate) struct HigherRankedLifetimeError { # [subdiagnostic] pub cause : Option < HigherRankedErrorCause > , # [primary_span] pub span : Span , }
    };
}

HigherRankedLifetimeError!()