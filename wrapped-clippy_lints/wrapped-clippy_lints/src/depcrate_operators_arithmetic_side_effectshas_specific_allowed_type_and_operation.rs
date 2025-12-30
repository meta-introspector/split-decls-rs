// Generated macro for has_specific_allowed_type_and_operation (function)
macro_rules! Depcrate_operators_arithmetic_side_effectshas_specific_allowed_type_and_operation {
() => {
// Module: crate::operators::arithmetic_side_effects
// Provides: {"has_specific_allowed_type_and_operation"}
// Dependencies: {}
# [doc = " Verifies built-in types that have specific allowed operations"] fn has_specific_allowed_type_and_operation < 'tcx > (cx : & LateContext < 'tcx > , lhs_ty : Ty < 'tcx > , op : hir :: BinOpKind , rhs_ty : Ty < 'tcx > ,) -> bool { let is_div_or_rem = matches ! (op , hir :: BinOpKind :: Div | hir :: BinOpKind :: Rem) ; let is_sat_or_wrap = | ty : Ty < '_ > | matches ! (ty . opt_diag_name (cx) , Some (sym :: Saturating | sym :: Wrapping)) ; if is_non_zero_u (cx , rhs_ty) && is_div_or_rem { return true ; } if is_sat_or_wrap (lhs_ty) { return ! is_div_or_rem ; } false }
};
}
