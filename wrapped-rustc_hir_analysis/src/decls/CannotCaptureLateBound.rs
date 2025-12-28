macro_rules! CannotCaptureLateBound {
    () => {
        # [derive (Diagnostic)] pub (crate) enum CannotCaptureLateBound { # [diag (hir_analysis_cannot_capture_late_bound_ty)] Type { # [primary_span] use_span : Span , # [label] def_span : Span , what : & 'static str , } , # [diag (hir_analysis_cannot_capture_late_bound_const)] Const { # [primary_span] use_span : Span , # [label] def_span : Span , what : & 'static str , } , # [diag (hir_analysis_cannot_capture_late_bound_lifetime)] Lifetime { # [primary_span] use_span : Span , # [label] def_span : Span , what : & 'static str , } , }
    };
}

CannotCaptureLateBound!();