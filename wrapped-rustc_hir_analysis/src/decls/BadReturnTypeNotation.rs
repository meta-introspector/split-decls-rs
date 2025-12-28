macro_rules! BadReturnTypeNotation {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_bad_return_type_notation_position)] pub (crate) struct BadReturnTypeNotation { # [primary_span] pub span : Span , }
    };
}

BadReturnTypeNotation!()