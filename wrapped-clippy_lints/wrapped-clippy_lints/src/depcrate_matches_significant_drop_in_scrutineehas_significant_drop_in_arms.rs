// Generated macro for has_significant_drop_in_arms (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineehas_significant_drop_in_arms {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"has_significant_drop_in_arms"}
// Dependencies: {}
fn has_significant_drop_in_arms < 'tcx > (cx : & LateContext < 'tcx > , arms : & [& 'tcx Expr < '_ >]) -> FxIndexSet < Span > { let mut helper = ArmSigDropHelper :: new (cx) ; for arm in arms { helper . visit_expr (arm) ; } helper . found_sig_drop_spans }
};
}
