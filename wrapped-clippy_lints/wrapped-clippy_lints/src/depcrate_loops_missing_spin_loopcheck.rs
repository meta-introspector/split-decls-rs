// Generated macro for check (function)
macro_rules! Depcrate_loops_missing_spin_loopcheck {
() => {
// Module: crate::loops::missing_spin_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , cond : & 'tcx Expr < '_ > , body : & 'tcx Expr < '_ >) { if let ExprKind :: Block (Block { stmts : [] , expr : None , .. } , _ ,) = body . kind && let ExprKind :: MethodCall (method , callee , ..) = unpack_cond (cond) . kind && [sym :: load , sym :: compare_exchange , sym :: compare_exchange_weak] . contains (& method . ident . name) && let callee_ty = cx . typeck_results () . expr_ty (callee) && callee_ty . is_diag_item (cx , sym :: AtomicBool) && let Some (std_or_core) = std_or_core (cx) { span_lint_and_sugg (cx , MISSING_SPIN_LOOP , body . span , "busy-waiting loop should at least have a spin loop hint" , "try" , format ! ("{{ {std_or_core}::hint::spin_loop() }}") , Applicability :: MachineApplicable ,) ; } }
};
}
