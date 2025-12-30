// Generated macro for NonBindingLet (enum)
macro_rules! Depcrate_lintsNonBindingLet {
() => {
// Module: crate::lints
// Provides: {"NonBindingLet"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum NonBindingLet { # [diag (lint_non_binding_let_on_sync_lock)] SyncLock { # [label] pat : Span , # [subdiagnostic] sub : NonBindingLetSub , } , # [diag (lint_non_binding_let_on_drop_type)] DropType { # [subdiagnostic] sub : NonBindingLetSub , } , }
};
}
