macro_rules! UnusedLabel {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_label)] pub (crate) struct UnusedLabel ;
    };
}

UnusedLabel!();