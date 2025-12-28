macro_rules! ReturnTypeNotationEqualityBound {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_return_type_notation_equality_bound)] pub (crate) struct ReturnTypeNotationEqualityBound { # [primary_span] pub span : Span , }
    };
}

ReturnTypeNotationEqualityBound!();