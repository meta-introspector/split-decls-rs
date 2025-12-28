macro_rules! NegativeBoundUnsupported {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_negative_bound_not_supported)] pub (crate) struct NegativeBoundUnsupported { # [primary_span] pub span : Span , }
    };
}

NegativeBoundUnsupported!();