macro_rules! LateBoundInApit {
    () => {
        # [derive (Diagnostic)] pub (crate) enum LateBoundInApit { # [diag (hir_analysis_late_bound_type_in_apit)] Type { # [primary_span] span : Span , # [label] param_span : Span , } , # [diag (hir_analysis_late_bound_const_in_apit)] Const { # [primary_span] span : Span , # [label] param_span : Span , } , # [diag (hir_analysis_late_bound_lifetime_in_apit)] Lifetime { # [primary_span] span : Span , # [label] param_span : Span , } , }
    };
}

LateBoundInApit!();