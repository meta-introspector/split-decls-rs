// Generated macro for pow (function)
macro_rules! Depcrate_float_powpow {
() => {
// Module: crate::float::pow
// Provides: {"pow"}
// Dependencies: {}
# [doc = " Returns `a` raised to the power `b`"] fn pow < F : Float > (a : F , b : i32) -> F { let mut a = a ; let recip = b < 0 ; let mut pow = Int :: abs_diff (b , 0) ; let mut mul = F :: ONE ; loop { if (pow & 1) != 0 { mul *= a ; } pow >>= 1 ; if pow == 0 { break ; } a *= a ; } if recip { F :: ONE / mul } else { mul } }
};
}
