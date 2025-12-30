// Generated macro for cmp (function)
macro_rules! Depcrate_float_cmpcmp {
() => {
// Module: crate::float::cmp
// Provides: {"cmp"}
// Dependencies: {}
fn cmp < F : Float > (a : F , b : F) -> Result { let one = F :: Int :: ONE ; let zero = F :: Int :: ZERO ; let szero = F :: SignedInt :: ZERO ; let sign_bit = F :: SIGN_MASK as F :: Int ; let abs_mask = sign_bit - one ; let exponent_mask = F :: EXP_MASK ; let inf_rep = exponent_mask ; let a_rep = a . to_bits () ; let b_rep = b . to_bits () ; let a_abs = a_rep & abs_mask ; let b_abs = b_rep & abs_mask ; if a_abs > inf_rep || b_abs > inf_rep { return Result :: Unordered ; } if a_abs | b_abs == zero { return Result :: Equal ; } let a_srep = a . to_bits_signed () ; let b_srep = b . to_bits_signed () ; if a_srep & b_srep >= szero { if a_srep < b_srep { Result :: Less } else if a_srep == b_srep { Result :: Equal } else { Result :: Greater } } else if a_srep > b_srep { Result :: Less } else if a_srep == b_srep { Result :: Equal } else { Result :: Greater } }
};
}
