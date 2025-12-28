macro_rules! ConstWithoutBody {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_const_without_body)] pub (crate) struct ConstWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
    };
}

ConstWithoutBody!()