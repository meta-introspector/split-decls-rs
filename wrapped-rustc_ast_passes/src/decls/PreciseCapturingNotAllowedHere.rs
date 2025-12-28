macro_rules! PreciseCapturingNotAllowedHere {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_precise_capturing_not_allowed_here)] pub (crate) struct PreciseCapturingNotAllowedHere { # [primary_span] pub span : Span , pub loc : & 'static str , }
    };
}

PreciseCapturingNotAllowedHere!()