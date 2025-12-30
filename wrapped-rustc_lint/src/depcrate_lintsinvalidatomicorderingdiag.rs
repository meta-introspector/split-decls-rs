// Generated macro for InvalidAtomicOrderingDiag (struct)
macro_rules! Depcrate_lintsInvalidAtomicOrderingDiag {
() => {
// Module: crate::lints
// Provides: {"InvalidAtomicOrderingDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_atomic_ordering_invalid)] # [help] pub (crate) struct InvalidAtomicOrderingDiag { pub method : Symbol , # [label] pub fail_order_arg_span : Span , }
};
}
