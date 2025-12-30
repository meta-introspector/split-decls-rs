// Generated macro for pow_call_result_sign (function)
macro_rules! Depcrate_casts_cast_sign_losspow_call_result_sign {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"pow_call_result_sign"}
// Dependencies: {}
# [doc = " Return the sign of the `pow` call's result, ignoring overflow."] # [doc = ""] # [doc = " If the base is positive, the result is always positive."] # [doc = " If the exponent is a even number, the result is always positive,"] # [doc = " Otherwise, if the base is negative, and the exponent is an odd number, the result is always"] # [doc = " negative."] # [doc = ""] # [doc = " Otherwise, returns [`Sign::Uncertain`]."] fn pow_call_result_sign (cx : & LateContext < '_ > , base : & Expr < '_ > , exponent : & Expr < '_ >) -> Sign { let base_sign = expr_sign (cx , base , None) ; let exponent_val = get_const_unsigned_int_eval (cx , exponent , None) ; let exponent_is_even = exponent_val . map (| val | val . is_multiple_of (2)) ; match (base_sign , exponent_is_even) { (Sign :: ZeroOrPositive , _) | (_ , Some (true)) => Sign :: ZeroOrPositive , (Sign :: Negative , Some (false)) => Sign :: Negative , (Sign :: Negative | Sign :: Uncertain , None) | (Sign :: Uncertain , Some (false)) => Sign :: Uncertain , } }
};
}
