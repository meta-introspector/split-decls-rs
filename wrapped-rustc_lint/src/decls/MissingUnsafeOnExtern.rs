macro_rules! MissingUnsafeOnExtern {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_missing_unsafe_on_extern)] pub (crate) struct MissingUnsafeOnExtern { # [suggestion (code = "unsafe " , applicability = "machine-applicable")] pub suggestion : Span , }
    };
}

MissingUnsafeOnExtern!()