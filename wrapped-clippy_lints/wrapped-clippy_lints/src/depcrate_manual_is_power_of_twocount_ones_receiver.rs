// Generated macro for count_ones_receiver (function)
macro_rules! Depcrate_manual_is_power_of_twocount_ones_receiver {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"count_ones_receiver"}
// Dependencies: {}
# [doc = " Return the unsigned integer receiver of `.count_ones()` or the argument of"] # [doc = " `<int-type>::count_ones(…)`."] fn count_ones_receiver < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { let (method , ty , receiver) = if let ExprKind :: MethodCall (method_name , receiver , [] , _) = expr . kind { (method_name , cx . typeck_results () . expr_ty_adjusted (receiver) , receiver) } else if let ExprKind :: Call (func , [arg]) = expr . kind && let ExprKind :: Path (QPath :: TypeRelative (ty , func_name)) = func . kind { (func_name , ty_from_hir_ty (cx , ty) , arg) } else { return None ; } ; (method . ident . name == sym :: count_ones && matches ! (ty . kind () , ty :: Uint (_))) . then_some (receiver) }
};
}
