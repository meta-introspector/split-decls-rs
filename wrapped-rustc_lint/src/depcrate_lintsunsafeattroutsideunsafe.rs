// Generated macro for UnsafeAttrOutsideUnsafe (struct)
macro_rules! Depcrate_lintsUnsafeAttrOutsideUnsafe {
() => {
// Module: crate::lints
// Provides: {"UnsafeAttrOutsideUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unsafe_attr_outside_unsafe)] pub (crate) struct UnsafeAttrOutsideUnsafe { # [label] pub span : Span , # [subdiagnostic] pub suggestion : UnsafeAttrOutsideUnsafeSuggestion , }
};
}
