// Generated macro for expr_sign (function)
macro_rules! Depcrate_casts_cast_sign_lossexpr_sign {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"expr_sign"}
// Dependencies: {}
fn expr_sign < 'cx , 'tcx > (cx : & LateContext < 'cx > , mut expr : & 'tcx Expr < 'tcx > , ty : impl Into < Option < Ty < 'cx > > >) -> Sign { if let Some (val) = get_const_signed_int_eval (cx , expr , ty) { return if val >= 0 { Sign :: ZeroOrPositive } else { Sign :: Negative } ; } if let Some (_val) = get_const_unsigned_int_eval (cx , expr , None) { return Sign :: ZeroOrPositive ; } if let ExprKind :: MethodCall (path , caller , args , ..) = expr . kind { let mut method_name = path . ident . name ; while let Some (& found_name) = METHODS_UNWRAP . iter () . find (| & name | & method_name == name) && let Some (arglist) = method_chain_args (expr , & [found_name]) && let ExprKind :: MethodCall (inner_path , recv , ..) = & arglist [0] . 0 . kind { method_name = inner_path . ident . name ; expr = recv ; } if METHODS_POW . contains (& method_name) && let [arg] = args { return pow_call_result_sign (cx , caller , arg) ; } else if METHODS_RET_POSITIVE . contains (& method_name) { return Sign :: ZeroOrPositive ; } } Sign :: Uncertain }
};
}
