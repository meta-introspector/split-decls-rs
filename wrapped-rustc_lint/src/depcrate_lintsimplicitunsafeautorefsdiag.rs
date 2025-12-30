// Generated macro for ImplicitUnsafeAutorefsDiag (struct)
macro_rules! Depcrate_lintsImplicitUnsafeAutorefsDiag {
() => {
// Module: crate::lints
// Provides: {"ImplicitUnsafeAutorefsDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_implicit_unsafe_autorefs)] # [note] pub (crate) struct ImplicitUnsafeAutorefsDiag < 'a > { # [label (lint_raw_ptr)] pub raw_ptr_span : Span , pub raw_ptr_ty : Ty < 'a > , # [subdiagnostic] pub origin : ImplicitUnsafeAutorefsOrigin < 'a > , # [subdiagnostic] pub method : Option < ImplicitUnsafeAutorefsMethodNote > , # [subdiagnostic] pub suggestion : ImplicitUnsafeAutorefsSuggestion , }
};
}
