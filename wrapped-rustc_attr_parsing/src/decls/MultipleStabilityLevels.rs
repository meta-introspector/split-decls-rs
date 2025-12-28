macro_rules! MultipleStabilityLevels {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_multiple_stability_levels , code = E0544)] pub (crate) struct MultipleStabilityLevels { # [primary_span] pub span : Span , }
    };
}

MultipleStabilityLevels!()