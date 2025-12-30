// Generated macro for DropRefDiag (struct)
macro_rules! Depcrate_lintsDropRefDiag {
() => {
// Module: crate::lints
// Provides: {"DropRefDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_dropping_references)] pub (crate) struct DropRefDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
};
}
