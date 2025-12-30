// Generated macro for DeprecatedLintNameFromCommandLine (struct)
macro_rules! Depcrate_lintsDeprecatedLintNameFromCommandLine {
() => {
// Module: crate::lints
// Provides: {"DeprecatedLintNameFromCommandLine"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_deprecated_lint_name)] # [help] pub (crate) struct DeprecatedLintNameFromCommandLine < 'a > { pub name : String , pub replace : & 'a str , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
};
}
