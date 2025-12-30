// Generated macro for unord (function)
macro_rules! Depcrate_float_cmpunord {
() => {
// Module: crate::float::cmp
// Provides: {"unord"}
// Dependencies: {}
fn unord < F : Float > (a : F , b : F) -> bool { let one = F :: Int :: ONE ; let sign_bit = F :: SIGN_MASK as F :: Int ; let abs_mask = sign_bit - one ; let exponent_mask = F :: EXP_MASK ; let inf_rep = exponent_mask ; let a_rep = a . to_bits () ; let b_rep = b . to_bits () ; let a_abs = a_rep & abs_mask ; let b_abs = b_rep & abs_mask ; a_abs > inf_rep || b_abs > inf_rep }
};
}
