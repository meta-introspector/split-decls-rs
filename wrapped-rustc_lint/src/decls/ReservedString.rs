macro_rules! ReservedString {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_reserved_string)] pub (crate) struct ReservedString { # [suggestion (code = " " , applicability = "machine-applicable")] pub suggestion : Span , }
    };
}

ReservedString!()