// Generated macro for check (function)
macro_rules! Depcrate_methods_swap_with_temporarycheck {
() => {
// Module: crate::methods::swap_with_temporary
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > , func : & Expr < '_ > , args : & 'tcx [Expr < '_ >]) { if let ExprKind :: Path (QPath :: Resolved (_ , func_path)) = func . kind && let Some (func_def_id) = func_path . res . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: mem_swap , func_def_id) { match (ArgKind :: new (cx , & args [0]) , ArgKind :: new (cx , & args [1])) { (ArgKind :: RefMutToTemp (left_temp) , ArgKind :: RefMutToTemp (right_temp)) => { emit_lint_useless (cx , expr , & args [0] , & args [1] , left_temp , right_temp) ; } , (ArgKind :: RefMutToTemp (left_temp) , right) => emit_lint_assign (cx , expr , & right , & args [0] , left_temp) , (left , ArgKind :: RefMutToTemp (right_temp)) => emit_lint_assign (cx , expr , & left , & args [1] , right_temp) , _ => { } , } } }
};
}
