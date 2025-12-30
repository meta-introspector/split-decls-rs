// Generated macro for InvalidFromUtf8Diag (enum)
macro_rules! Depcrate_lintsInvalidFromUtf8Diag {
() => {
// Module: crate::lints
// Provides: {"InvalidFromUtf8Diag"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum InvalidFromUtf8Diag { # [diag (lint_invalid_from_utf8_unchecked)] Unchecked { method : String , valid_up_to : usize , # [label] label : Span , } , # [diag (lint_invalid_from_utf8_checked)] Checked { method : String , valid_up_to : usize , # [label] label : Span , } , }
};
}
