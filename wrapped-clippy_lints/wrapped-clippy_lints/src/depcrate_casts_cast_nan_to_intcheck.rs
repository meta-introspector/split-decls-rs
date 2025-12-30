// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_nan_to_intcheck {
() => {
// Module: crate::casts::cast_nan_to_int
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , from_ty : Ty < '_ > , to_ty : Ty < '_ >) { if from_ty . is_floating_point () && to_ty . is_integral () && is_known_nan (cx , cast_expr) { span_lint_and_note (cx , CAST_NAN_TO_INT , expr . span , format ! ("casting a known NaN to {to_ty}") , None , "this always evaluates to 0" ,) ; } }
};
}
