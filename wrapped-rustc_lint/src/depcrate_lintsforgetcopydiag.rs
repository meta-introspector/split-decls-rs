// Generated macro for ForgetCopyDiag (struct)
macro_rules! Depcrate_lintsForgetCopyDiag {
() => {
// Module: crate::lints
// Provides: {"ForgetCopyDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_forgetting_copy_types)] pub (crate) struct ForgetCopyDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
};
}
