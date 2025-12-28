macro_rules! DuplicateMatcherBinding {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_duplicate_matcher_binding)] pub (crate) struct DuplicateMatcherBinding ;
    };
}

DuplicateMatcherBinding!();