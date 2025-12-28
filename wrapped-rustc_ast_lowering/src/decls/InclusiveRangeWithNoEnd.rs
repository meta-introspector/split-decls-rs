macro_rules! InclusiveRangeWithNoEnd {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_inclusive_range_with_no_end)] pub (crate) struct InclusiveRangeWithNoEnd { # [primary_span] pub span : Span , }
    };
}

InclusiveRangeWithNoEnd!();