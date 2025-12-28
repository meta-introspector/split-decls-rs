macro_rules! PatternInForeign {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_pattern_in_foreign , code = E0130)] pub (crate) struct PatternInForeign { # [primary_span] # [label] pub span : Span , }
    };
}

PatternInForeign!();