macro_rules! ReservedPrefix {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_reserved_prefix)] pub (crate) struct ReservedPrefix { # [label] pub label : Span , # [suggestion (code = " " , applicability = "machine-applicable")] pub suggestion : Span , pub prefix : String , }
    };
}

ReservedPrefix!();