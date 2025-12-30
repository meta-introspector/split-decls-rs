// Generated macro for peel_raw_casts (function)
macro_rules! Depcrate_ptrpeel_raw_casts {
() => {
// Module: crate::ptr
// Provides: {"peel_raw_casts"}
// Dependencies: {}
fn peel_raw_casts < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , expr_ty : Ty < 'tcx >) -> (& 'tcx Expr < 'tcx > , bool) { if ! expr . span . from_expansion () && let ExprKind :: Cast (inner , _) = expr . kind && let ty :: RawPtr (target_ty , _) = expr_ty . kind () && let inner_ty = cx . typeck_results () . expr_ty (inner) && let ty :: RawPtr (inner_target_ty , _) | ty :: Ref (_ , inner_target_ty , _) = inner_ty . kind () && target_ty == inner_target_ty { (peel_raw_casts (cx , inner , inner_ty) . 0 , true) } else { (expr , false) } }
};
}
