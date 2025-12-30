// Generated macro for get_const_unsigned_int_eval (function)
macro_rules! Depcrate_casts_cast_sign_lossget_const_unsigned_int_eval {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"get_const_unsigned_int_eval"}
// Dependencies: {}
fn get_const_unsigned_int_eval < 'cx > (cx : & LateContext < 'cx > , expr : & Expr < '_ > , ty : impl Into < Option < Ty < 'cx > > > ,) -> Option < u128 > { let ty = ty . into () . unwrap_or_else (| | cx . typeck_results () . expr_ty (expr)) ; if let Constant :: Int (n) = ConstEvalCtxt :: new (cx) . eval (expr) ? && let ty :: Uint (_ity) = * ty . kind () { return Some (n) ; } None }
};
}
