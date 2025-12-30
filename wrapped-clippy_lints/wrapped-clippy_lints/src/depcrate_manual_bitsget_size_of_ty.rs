// Generated macro for get_size_of_ty (function)
macro_rules! Depcrate_manual_bitsget_size_of_ty {
() => {
// Module: crate::manual_bits
// Provides: {"get_size_of_ty"}
// Dependencies: {}
fn get_size_of_ty < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < (Span , Ty < 'tcx >) > { if let ExprKind :: Call (count_func , []) = expr . kind && let ExprKind :: Path (ref count_func_qpath) = count_func . kind && let QPath :: Resolved (_ , count_func_path) = count_func_qpath && let Some (segment_zero) = count_func_path . segments . first () && let Some (args) = segment_zero . args && let Some (real_ty_span) = args . args . first () . map (GenericArg :: span) && let Some (def_id) = cx . qpath_res (count_func_qpath , count_func . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: mem_size_of , def_id) { cx . typeck_results () . node_args (count_func . hir_id) . types () . next () . map (| resolved_ty | (real_ty_span , resolved_ty)) } else { None } }
};
}
