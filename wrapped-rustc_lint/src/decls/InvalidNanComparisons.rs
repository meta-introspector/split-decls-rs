macro_rules! deps {
    () => {
        InvalidNanComparisonsSuggestion!();
    };
}

macro_rules! InvalidNanComparisons {
    () => {
        deps!();
        # [derive (LintDiagnostic)] pub (crate) enum InvalidNanComparisons { # [diag (lint_invalid_nan_comparisons_eq_ne)] EqNe { # [subdiagnostic] suggestion : InvalidNanComparisonsSuggestion , } , # [diag (lint_invalid_nan_comparisons_lt_le_gt_ge)] LtLeGtGe , }
    };
}

InvalidNanComparisons!();