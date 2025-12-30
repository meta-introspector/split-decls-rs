// Generated macro for BuiltinUnreachablePub (struct)
macro_rules! Depcrate_lintsBuiltinUnreachablePub {
() => {
// Module: crate::lints
// Provides: {"BuiltinUnreachablePub"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_unreachable_pub)] pub (crate) struct BuiltinUnreachablePub < 'a > { pub what : & 'a str , pub new_vis : & 'a str , # [suggestion (code = "{new_vis}")] pub suggestion : (Span , Applicability) , # [help] pub help : bool , }
};
}
