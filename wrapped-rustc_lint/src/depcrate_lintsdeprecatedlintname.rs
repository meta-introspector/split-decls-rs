// Generated macro for DeprecatedLintName (struct)
macro_rules! Depcrate_lintsDeprecatedLintName {
() => {
// Module: crate::lints
// Provides: {"DeprecatedLintName"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_deprecated_lint_name)] pub (crate) struct DeprecatedLintName < 'a > { pub name : String , # [suggestion (code = "{replace}" , applicability = "machine-applicable")] pub suggestion : Span , pub replace : & 'a str , }
};
}
