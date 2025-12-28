macro_rules! NeverPatternWithBody {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_never_pattern_with_body)] pub (crate) struct NeverPatternWithBody { # [primary_span] # [label] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , }
    };
}

NeverPatternWithBody!();