// Generated macro for get_callee_generic_args_and_args (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedget_callee_generic_args_and_args {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"get_callee_generic_args_and_args"}
// Dependencies: {}
# [doc = " Checks whether an expression is a function or method call and, if so, returns its `DefId`,"] # [doc = " `GenericArgs`, and arguments."] fn get_callee_generic_args_and_args < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > ,) -> Option < (DefId , GenericArgsRef < 'tcx > , Option < & 'tcx Expr < 'tcx > > , & 'tcx [Expr < 'tcx >] ,) > { if let ExprKind :: Call (callee , args) = expr . kind && let callee_ty = cx . typeck_results () . expr_ty (callee) && let ty :: FnDef (callee_def_id , _) = callee_ty . kind () { let generic_args = cx . typeck_results () . node_args (callee . hir_id) ; return Some ((* callee_def_id , generic_args , None , args)) ; } if let ExprKind :: MethodCall (_ , recv , args , _) = expr . kind && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { let generic_args = cx . typeck_results () . node_args (expr . hir_id) ; return Some ((method_def_id , generic_args , Some (recv) , args)) ; } None }
};
}
