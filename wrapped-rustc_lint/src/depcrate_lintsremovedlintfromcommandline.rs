// Generated macro for RemovedLintFromCommandLine (struct)
macro_rules! Depcrate_lintsRemovedLintFromCommandLine {
() => {
// Module: crate::lints
// Provides: {"RemovedLintFromCommandLine"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_removed_lint)] pub (crate) struct RemovedLintFromCommandLine < 'a > { pub name : & 'a str , pub reason : & 'a str , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
};
}
