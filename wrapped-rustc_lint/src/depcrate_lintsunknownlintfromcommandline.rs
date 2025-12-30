// Generated macro for UnknownLintFromCommandLine (struct)
macro_rules! Depcrate_lintsUnknownLintFromCommandLine {
() => {
// Module: crate::lints
// Provides: {"UnknownLintFromCommandLine"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unknown_lint , code = E0602)] pub (crate) struct UnknownLintFromCommandLine < 'a > { pub name : String , # [subdiagnostic] pub suggestion : Option < UnknownLintSuggestion > , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
};
}
