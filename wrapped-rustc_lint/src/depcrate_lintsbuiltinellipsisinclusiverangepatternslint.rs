// Generated macro for BuiltinEllipsisInclusiveRangePatternsLint (enum)
macro_rules! Depcrate_lintsBuiltinEllipsisInclusiveRangePatternsLint {
() => {
// Module: crate::lints
// Provides: {"BuiltinEllipsisInclusiveRangePatternsLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum BuiltinEllipsisInclusiveRangePatternsLint { # [diag (lint_builtin_ellipsis_inclusive_range_patterns)] Parenthesise { # [suggestion (code = "{replace}" , applicability = "machine-applicable")] suggestion : Span , replace : String , } , # [diag (lint_builtin_ellipsis_inclusive_range_patterns)] NonParenthesise { # [suggestion (style = "short" , code = "..=" , applicability = "machine-applicable")] suggestion : Span , } , }
};
}
