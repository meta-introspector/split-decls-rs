// Generated macro for as_ulp_steps (function)
macro_rules! Depcrate_numas_ulp_steps {
() => {
// Module: crate::num
// Provides: {"as_ulp_steps"}
// Dependencies: {}
# [doc = " Return the (signed) number of steps from zero to `x`."] fn as_ulp_steps < F : Float > (x : F) -> Option < F :: SignedInt > { let s = x . to_bits_signed () ; let val = if s >= F :: SignedInt :: ZERO { s } else { F :: SignedInt :: MIN - s } ; (! x . is_nan ()) . then_some (val) }
};
}
