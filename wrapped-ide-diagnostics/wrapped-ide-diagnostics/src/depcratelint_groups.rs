// Generated macro for lint_groups (function)
macro_rules! Depcratelint_groups {
() => {
// Module: crate
// Provides: {"lint_groups"}
// Dependencies: {}
fn lint_groups (lint : & DiagnosticCode , edition : Edition) -> LintGroups { let (groups , inside_warnings) = match lint { DiagnosticCode :: RustcLint (name) => { let lint = & RUSTC_LINTS [name] ; let inside_warnings = default_lint_severity (lint . lint , edition) == Severity :: Warning ; (& lint . groups , inside_warnings) } DiagnosticCode :: Clippy (name) => { let lint = & CLIPPY_LINTS [name] ; let inside_warnings = default_lint_severity (lint . lint , edition) == Severity :: Warning ; (& lint . groups , inside_warnings) } _ => panic ! ("non-lint passed to `handle_lints()`") , } ; LintGroups { groups , inside_warnings } }
};
}
