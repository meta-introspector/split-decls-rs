// Generated macro for get_vec_init_kind (function)
macro_rules! Depcrate_higherget_vec_init_kind {
() => {
// Module: crate::higher
// Provides: {"get_vec_init_kind"}
// Dependencies: {}
# [doc = " Checks if the given expression is an initialization of `Vec` and returns its kind."] pub fn get_vec_init_kind < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < VecInitKind > { if let ExprKind :: Call (func , args) = expr . kind { match func . kind { ExprKind :: Path (QPath :: TypeRelative (ty , name)) if cx . typeck_results () . node_type (ty . hir_id) . is_diag_item (cx , sym :: Vec) => { if name . ident . name == sym :: new { return Some (VecInitKind :: New) ; } else if name . ident . name == symbol :: kw :: Default { return Some (VecInitKind :: Default) ; } else if name . ident . name == sym :: with_capacity { let arg = args . first () ? ; return match ConstEvalCtxt :: new (cx) . eval_local (arg , expr . span . ctxt ()) { Some (Constant :: Int (num)) => Some (VecInitKind :: WithConstCapacity (num)) , _ => Some (VecInitKind :: WithExprCapacity (arg . hir_id)) , } ; } } , ExprKind :: Path (QPath :: Resolved (_ , path)) if cx . tcx . is_diagnostic_item (sym :: default_fn , path . res . opt_def_id () ?) && cx . typeck_results () . expr_ty (expr) . is_diag_item (cx , sym :: Vec) => { return Some (VecInitKind :: Default) ; } , _ => () , } } None }
};
}
