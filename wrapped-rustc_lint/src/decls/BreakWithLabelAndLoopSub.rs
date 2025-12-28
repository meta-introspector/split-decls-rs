macro_rules! BreakWithLabelAndLoopSub {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "machine-applicable")] pub (crate) struct BreakWithLabelAndLoopSub { # [suggestion_part (code = "(")] pub left : Span , # [suggestion_part (code = ")")] pub right : Span , }
    };
}

BreakWithLabelAndLoopSub!()