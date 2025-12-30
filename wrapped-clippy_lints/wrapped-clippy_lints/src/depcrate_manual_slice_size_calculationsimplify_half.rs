// Generated macro for simplify_half (function)
macro_rules! Depcrate_manual_slice_size_calculationsimplify_half {
() => {
// Module: crate::manual_slice_size_calculation
// Provides: {"simplify_half"}
// Dependencies: {}
fn simplify_half < 'tcx > (cx : & LateContext < 'tcx > , expr1 : & 'tcx Expr < 'tcx > , expr2 : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , usize) > { if ! expr1 . span . from_expansion () && let ExprKind :: MethodCall (method_path , receiver , [] , _) = expr1 . kind && method_path . ident . name == sym :: len && let receiver_ty = cx . typeck_results () . expr_ty (receiver) && let (receiver_ty , refs_count , _) = peel_and_count_ty_refs (receiver_ty) && let ty :: Slice (ty1) = receiver_ty . kind () && let ExprKind :: Call (func , []) = expr2 . kind && let ExprKind :: Path (ref func_qpath) = func . kind && let Some (def_id) = cx . qpath_res (func_qpath , func . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: mem_size_of , def_id) && let Some (ty2) = cx . typeck_results () . node_args (func . hir_id) . types () . next () && * ty1 == ty2 { Some ((receiver , refs_count)) } else { None } }
};
}
