macro_rules! RawPrefix {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_raw_prefix)] pub (crate) struct RawPrefix { # [label] pub label : Span , # [suggestion (code = " " , applicability = "machine-applicable")] pub suggestion : Span , }
    };
}

RawPrefix!()