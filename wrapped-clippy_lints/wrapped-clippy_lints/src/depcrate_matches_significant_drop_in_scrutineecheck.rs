// Generated macro for check (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineecheck {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"check"}
// Dependencies: {}
fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , scrutinee : & 'tcx Expr < '_ > , arms : & [& 'tcx Expr < '_ >] , message : & 'static str , sugg : Suggestion ,) { let mut helper = SigDropHelper :: new (cx) ; let suggestions = helper . find_sig_drop (scrutinee) ; for found in suggestions { span_lint_and_then (cx , SIGNIFICANT_DROP_IN_SCRUTINEE , found . found_span , message , | diag | { match sugg { Suggestion :: Emit => set_suggestion (diag , cx , expr , found) , Suggestion :: DontEmit => () , } let s = Span :: new (expr . span . hi () , expr . span . hi () , expr . span . ctxt () , None) ; diag . span_label (s , "temporary lives until here") ; for span in has_significant_drop_in_arms (cx , arms) { diag . span_label (span , "another value with significant `Drop` created here") ; } diag . note ("this might lead to deadlocks or other unexpected behavior") ; }) ; } }
};
}
