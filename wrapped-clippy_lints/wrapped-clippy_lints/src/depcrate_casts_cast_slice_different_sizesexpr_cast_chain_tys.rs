// Generated macro for expr_cast_chain_tys (function)
macro_rules! Depcrate_casts_cast_slice_different_sizesexpr_cast_chain_tys {
() => {
// Module: crate::casts::cast_slice_different_sizes
// Provides: {"expr_cast_chain_tys"}
// Dependencies: {}
# [doc = " Returns a `CastChainInfo` with the left-most cast in the chain and the original ptr T and final"] # [doc = " ptr U if the expression is composed of casts."] # [doc = " Returns None if the expr is not a Cast"] fn expr_cast_chain_tys < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < CastChainInfo < 'tcx > > { if let ExprKind :: Cast (cast_expr , _cast_to_hir_ty) = expr . peel_blocks () . kind { let cast_to = cx . typeck_results () . expr_ty (expr) ; let to_slice_ty = get_raw_slice_ty_mut (cast_to) ? ; if let Some (prev_info) = expr_cast_chain_tys (cx , cast_expr) { Some (CastChainInfo { end_ty : to_slice_ty , .. prev_info }) } else { let cast_from = cx . typeck_results () . expr_ty (cast_expr) ; let from_slice_ty = get_raw_slice_ty_mut (cast_from) ? ; Some (CastChainInfo { left_cast : cast_expr , start_ty : from_slice_ty , end_ty : to_slice_ty , }) } } else { None } }
};
}
