// Generated macro for ReservedString (struct)
macro_rules! Depcrate_lintsReservedString {
() => {
// Module: crate::lints
// Provides: {"ReservedString"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_reserved_string)] pub (crate) struct ReservedString { # [suggestion (code = " " , applicability = "machine-applicable")] pub suggestion : Span , }
};
}
