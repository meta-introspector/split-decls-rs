// Generated macro for impl_698 (impl)
macro_rules! Depcrate_lintsimpl_698 {
() => {
// Module: crate::lints
// Provides: {"impl_698"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for BuiltinUngatedAsyncFnTrackCaller < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_ungated_async_fn_track_caller) ; diag . span_label (self . label , fluent :: lint_label) ; rustc_session :: parse :: add_feature_diagnostics (diag , self . session , sym :: async_fn_track_caller ,) ; } }
};
}
