use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn lint_groups(lint: &DiagnosticCode, edition: Edition) -> LintGroups {
    let (groups, inside_warnings) = match lint {
        DiagnosticCode::RustcLint(name) => {
            let lint = &RUSTC_LINTS[name];
            let inside_warnings = default_lint_severity(lint.lint, edition) == Severity::Warning;
            (&lint.groups, inside_warnings)
        }
        DiagnosticCode::Clippy(name) => {
            let lint = &CLIPPY_LINTS[name];
            let inside_warnings = default_lint_severity(lint.lint, edition) == Severity::Warning;
            (&lint.groups, inside_warnings)
        }
        _ => panic!("non-lint passed to `handle_lints()`"),
    };
    LintGroups {
        groups,
        inside_warnings,
    }
}
