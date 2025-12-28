macro_rules! NegativeBoundWithParentheticalNotation {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_negative_bound_with_parenthetical_notation)] pub (crate) struct NegativeBoundWithParentheticalNotation { # [primary_span] pub span : Span , }
    };
}

NegativeBoundWithParentheticalNotation!()