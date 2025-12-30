// Generated macro for check_method (function)
macro_rules! Depcrate_loops_unused_enumerate_indexcheck_method {
() => {
// Module: crate::loops::unused_enumerate_index
// Provides: {"check_method"}
// Dependencies: {}
pub (super) fn check_method < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx > , recv : & 'tcx Expr < 'tcx > , arg : & 'tcx Expr < 'tcx > ,) { if let ExprKind :: Closure (closure) = arg . kind && let body = cx . tcx . hir_body (closure . body) && let [param] = body . params && cx . ty_based_def (e) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let [input] = closure . fn_decl . inputs && ! arg . span . from_expansion () && ! input . span . from_expansion () && ! recv . span . from_expansion () && ! param . span . from_expansion () { let ty_spans = if let TyKind :: Tup ([_ , inner]) = input . kind { let Some (inner) = walk_span_to_context (inner . span , SyntaxContext :: root ()) else { return ; } ; Some ((input . span , inner)) } else { None } ; check (cx , recv , param . pat , ty_spans , body . value) ; } }
};
}
