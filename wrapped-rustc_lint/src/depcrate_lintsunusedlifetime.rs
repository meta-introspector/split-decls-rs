// Generated macro for UnusedLifetime (struct)
macro_rules! Depcrate_lintsUnusedLifetime {
() => {
// Module: crate::lints
// Provides: {"UnusedLifetime"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unused_lifetime)] pub (crate) struct UnusedLifetime { # [suggestion (code = "" , applicability = "machine-applicable")] pub deletion_span : Option < Span > , pub ident : Ident , }
};
}
