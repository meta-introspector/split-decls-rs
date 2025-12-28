macro_rules! NeverPatternWithGuard {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_never_pattern_with_guard)] pub (crate) struct NeverPatternWithGuard { # [primary_span] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , }
    };
}

NeverPatternWithGuard!();