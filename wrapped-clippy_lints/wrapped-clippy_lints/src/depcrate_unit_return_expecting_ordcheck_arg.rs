// Generated macro for check_arg (function)
macro_rules! Depcrate_unit_return_expecting_ordcheck_arg {
() => {
// Module: crate::unit_return_expecting_ord
// Provides: {"check_arg"}
// Dependencies: {}
fn check_arg < 'tcx > (cx : & LateContext < 'tcx > , arg : & 'tcx Expr < 'tcx >) -> Option < (Span , Option < Span >) > { if let ExprKind :: Closure (& Closure { body , fn_decl_span , .. }) = arg . kind && let ty :: Closure (_def_id , args) = & cx . typeck_results () . node_type (arg . hir_id) . kind () && let ret_ty = args . as_closure () . sig () . output () && let ty = cx . tcx . instantiate_bound_regions_with_erased (ret_ty) && ty . is_unit () { let body = cx . tcx . hir_body (body) ; if let ExprKind :: Block (block , _) = body . value . kind && block . expr . is_none () && let Some (stmt) = block . stmts . last () && let StmtKind :: Semi (_) = stmt . kind { let data = stmt . span . data () ; Some ((fn_decl_span , Some (data . with_lo (data . hi - BytePos (1))))) } else { Some ((fn_decl_span , None)) } } else { None } }
};
}
