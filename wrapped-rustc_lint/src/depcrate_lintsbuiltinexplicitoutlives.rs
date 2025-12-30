// Generated macro for BuiltinExplicitOutlives (struct)
macro_rules! Depcrate_lintsBuiltinExplicitOutlives {
() => {
// Module: crate::lints
// Provides: {"BuiltinExplicitOutlives"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_explicit_outlives)] pub (crate) struct BuiltinExplicitOutlives { pub count : usize , # [subdiagnostic] pub suggestion : BuiltinExplicitOutlivesSuggestion , }
};
}
