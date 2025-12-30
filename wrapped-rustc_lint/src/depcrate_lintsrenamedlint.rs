// Generated macro for RenamedLint (struct)
macro_rules! Depcrate_lintsRenamedLint {
() => {
// Module: crate::lints
// Provides: {"RenamedLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_renamed_lint)] pub (crate) struct RenamedLint < 'a > { pub name : & 'a str , pub replace : & 'a str , # [subdiagnostic] pub suggestion : RenamedLintSuggestion < 'a > , }
};
}
