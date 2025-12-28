macro_rules! InvalidNanComparisonsSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum InvalidNanComparisonsSuggestion { # [multipart_suggestion (lint_suggestion , style = "verbose" , applicability = "machine-applicable")] Spanful { # [suggestion_part (code = "!")] neg : Option < Span > , # [suggestion_part (code = ".is_nan()")] float : Span , # [suggestion_part (code = "")] nan_plus_binop : Span , } , # [help (lint_suggestion)] Spanless , }
    };
}

InvalidNanComparisonsSuggestion!();