// Generated macro for UnknownLint (struct)
macro_rules! Depcrate_lintsUnknownLint {
() => {
// Module: crate::lints
// Provides: {"UnknownLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unknown_lint)] pub (crate) struct UnknownLint { pub name : String , # [subdiagnostic] pub suggestion : Option < UnknownLintSuggestion > , }
};
}
