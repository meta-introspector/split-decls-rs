macro_rules! BuiltinConstNoMangle {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_const_no_mangle)] pub (crate) struct BuiltinConstNoMangle { # [suggestion (code = "pub static" , applicability = "machine-applicable")] pub suggestion : Span , }
    };
}

BuiltinConstNoMangle!();