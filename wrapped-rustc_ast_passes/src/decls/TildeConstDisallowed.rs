macro_rules! deps {
    () => {
        TildeConstReason!();
    };
}

macro_rules! TildeConstDisallowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_tilde_const_disallowed)] pub (crate) struct TildeConstDisallowed { # [primary_span] pub span : Span , # [subdiagnostic] pub reason : TildeConstReason , }
    };
}

TildeConstDisallowed!()