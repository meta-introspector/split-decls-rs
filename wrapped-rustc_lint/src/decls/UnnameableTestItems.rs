macro_rules! UnnameableTestItems {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unnameable_test_items)] pub (crate) struct UnnameableTestItems ;
    };
}

UnnameableTestItems!();