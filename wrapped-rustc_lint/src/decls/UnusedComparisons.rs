macro_rules! UnusedComparisons {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_comparisons)] pub (crate) struct UnusedComparisons ;
    };
}

UnusedComparisons!();