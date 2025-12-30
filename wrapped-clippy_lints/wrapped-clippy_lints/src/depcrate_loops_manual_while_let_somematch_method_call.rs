// Generated macro for match_method_call (function)
macro_rules! Depcrate_loops_manual_while_let_somematch_method_call {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"match_method_call"}
// Dependencies: {}
fn match_method_call < const ARGS_COUNT : usize > (cx : & LateContext < '_ > , expr : & Expr < '_ > , method : Symbol) -> bool { if let ExprKind :: MethodCall (_ , _ , args , _) = expr . kind && args . len () == ARGS_COUNT && let Some (id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { cx . tcx . is_diagnostic_item (method , id) } else { false } }
};
}
