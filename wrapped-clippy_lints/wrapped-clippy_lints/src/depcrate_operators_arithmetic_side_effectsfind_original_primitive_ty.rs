// Generated macro for find_original_primitive_ty (function)
macro_rules! Depcrate_operators_arithmetic_side_effectsfind_original_primitive_ty {
() => {
// Module: crate::operators::arithmetic_side_effects
// Provides: {"find_original_primitive_ty"}
// Dependencies: {}
# [doc = " Detects a type-casting conversion and returns the type of the original expression. For"] # [doc = " example, `let foo = u64::from(bar)`."] fn find_original_primitive_ty < 'tcx > (cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ >) -> Option < Ty < 'tcx > > { if let hir :: ExprKind :: Call (path , [arg]) = & expr . kind && path . res (cx) . opt_def_id () . is_diag_item (& cx . tcx , sym :: from_fn) { Some (cx . typeck_results () . expr_ty (arg)) } else { None } }
};
}
