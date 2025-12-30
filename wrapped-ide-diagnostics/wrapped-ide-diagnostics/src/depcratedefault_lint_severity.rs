// Generated macro for default_lint_severity (function)
macro_rules! Depcratedefault_lint_severity {
() => {
// Module: crate
// Provides: {"default_lint_severity"}
// Dependencies: {}
fn default_lint_severity (lint : & Lint , edition : Edition) -> Severity { if lint . deny_since . is_some_and (| e | edition >= e) { Severity :: Error } else if lint . warn_since . is_some_and (| e | edition >= e) { Severity :: Warning } else { lint . default_severity } }
};
}
