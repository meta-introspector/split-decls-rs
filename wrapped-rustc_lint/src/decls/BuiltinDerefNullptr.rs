macro_rules! BuiltinDerefNullptr {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_deref_nullptr)] pub (crate) struct BuiltinDerefNullptr { # [label] pub label : Span , }
    };
}

BuiltinDerefNullptr!();