macro_rules! StaticWithoutBody {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_static_without_body)] pub (crate) struct StaticWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <expr>;" , applicability = "has-placeholders")] pub replace_span : Span , }
    };
}

StaticWithoutBody!()