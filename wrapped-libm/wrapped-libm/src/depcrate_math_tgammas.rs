// Generated macro for s (function)
macro_rules! Depcrate_math_tgammas {
() => {
// Module: crate::math::tgamma
// Provides: {"s"}
// Dependencies: {}
fn s (x : f64) -> f64 { let mut num : f64 = 0.0 ; let mut den : f64 = 0.0 ; if x < 8.0 { for i in (0 ..= N) . rev () { num = num * x + i ! (SNUM , i) ; den = den * x + i ! (SDEN , i) ; } } else { for i in 0 ..= N { num = num / x + i ! (SNUM , i) ; den = den / x + i ! (SDEN , i) ; } } return num / den ; }
};
}
