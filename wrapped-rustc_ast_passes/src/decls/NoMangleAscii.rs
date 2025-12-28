macro_rules! NoMangleAscii {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_nomangle_ascii , code = E0754)] pub (crate) struct NoMangleAscii { # [primary_span] pub span : Span , }
    };
}

NoMangleAscii!();