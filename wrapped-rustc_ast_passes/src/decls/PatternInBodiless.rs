macro_rules! PatternInBodiless {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_pattern_in_bodiless , code = E0642)] pub (crate) struct PatternInBodiless { # [primary_span] # [label] pub span : Span , }
    };
}

PatternInBodiless!();