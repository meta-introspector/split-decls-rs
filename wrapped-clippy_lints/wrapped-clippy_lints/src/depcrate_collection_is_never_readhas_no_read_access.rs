// Generated macro for has_no_read_access (function)
macro_rules! Depcrate_collection_is_never_readhas_no_read_access {
() => {
// Module: crate::collection_is_never_read
// Provides: {"has_no_read_access"}
// Dependencies: {}
fn has_no_read_access < 'tcx , T : Visitable < 'tcx > > (cx : & LateContext < 'tcx > , id : HirId , block : T) -> bool { let mut has_access = false ; let mut has_read_access = false ; for_each_expr (cx , block , | expr | { if expr . res_local_id () != Some (id) { return ControlFlow :: Continue (()) ; } has_access = true ; if let Node :: Expr (parent) = cx . tcx . parent_hir_node (expr . hir_id) && let ExprKind :: Assign (lhs , ..) = parent . kind && lhs . res_local_id () == Some (id) { return ControlFlow :: Continue (()) ; } if let Node :: Expr (parent) = cx . tcx . parent_hir_node (expr . hir_id) && let ExprKind :: MethodCall (_ , receiver , args , _) = parent . kind && receiver . res_local_id () == Some (id) && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (parent . hir_id) && ! method_def_id . is_local () { let is_read_in_closure_arg = args . iter () . any (| arg | { if let ExprKind :: Closure (closure) = arg . kind && let Body { params : [param , ..] , value } = cx . tcx . hir_body (closure . body) { ! has_no_read_access (cx , param . hir_id , * value) } else { false } }) ; if is_read_in_closure_arg { has_read_access = true ; return ControlFlow :: Break (()) ; } if let Node :: Stmt (..) = cx . tcx . parent_hir_node (parent . hir_id) { return ControlFlow :: Continue (()) ; } if cx . typeck_results () . expr_ty (parent) . is_unit () { return ControlFlow :: Continue (()) ; } } has_read_access = true ; ControlFlow :: Break (()) }) ; has_access && ! has_read_access }
};
}
