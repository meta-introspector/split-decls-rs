macro_rules! AssocConstWithoutBody {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_assoc_const_without_body)] pub (crate) struct AssocConstWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
    };
}

AssocConstWithoutBody!()