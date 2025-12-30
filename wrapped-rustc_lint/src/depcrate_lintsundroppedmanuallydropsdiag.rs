// Generated macro for UndroppedManuallyDropsDiag (struct)
macro_rules! Depcrate_lintsUndroppedManuallyDropsDiag {
() => {
// Module: crate::lints
// Provides: {"UndroppedManuallyDropsDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_undropped_manually_drops)] pub (crate) struct UndroppedManuallyDropsDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub suggestion : UndroppedManuallyDropsSuggestion , }
};
}
