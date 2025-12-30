// Generated macro for ln_of_factorial (function)
macro_rules! Depcrate_hypergeometricln_of_factorial {
() => {
// Module: crate::hypergeometric
// Provides: {"ln_of_factorial"}
// Dependencies: {}
fn ln_of_factorial (v : f64) -> f64 { let v_3 = v + 3.0 ; let ln_fac = (v_3 + 0.5) * v_3 . ln () - v_3 + LOGSQRT2PI + 1.0 / (12.0 * v_3) ; ln_fac - ((v + 3.0) * (v + 2.0) * (v + 1.0)) . ln () }
};
}
