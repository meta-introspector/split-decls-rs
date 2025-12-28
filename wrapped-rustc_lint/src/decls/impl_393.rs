macro_rules! deps {
    () => {
        BuiltinUngatedAsyncFnTrackCaller!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for BuiltinUngatedAsyncFnTrackCaller < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_ungated_async_fn_track_caller) ; diag . span_label (self . label , fluent :: lint_label) ; rustc_session :: parse :: add_feature_diagnostics (diag , self . session , sym :: async_fn_track_caller ,) ; } }
    };
}

impl_393!()