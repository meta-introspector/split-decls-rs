// Generated macro for BuiltinConstNoMangle (struct)
macro_rules! Depcrate_lintsBuiltinConstNoMangle {
() => {
// Module: crate::lints
// Provides: {"BuiltinConstNoMangle"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_const_no_mangle)] pub (crate) struct BuiltinConstNoMangle { # [suggestion (code = "pub static" , applicability = "machine-applicable")] pub suggestion : Span , }
};
}
