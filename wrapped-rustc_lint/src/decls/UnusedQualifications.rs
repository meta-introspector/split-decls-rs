macro_rules! UnusedQualifications {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unnecessary_qualification)] pub (crate) struct UnusedQualifications { # [suggestion (style = "verbose" , code = "" , applicability = "machine-applicable")] pub removal_span : Span , }
    };
}

UnusedQualifications!()