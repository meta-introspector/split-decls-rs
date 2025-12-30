// Generated macro for brusselator_f (function)
macro_rules! Depcrate_unsfbrusselator_f {
() => {
// Module: crate::unsf
// Provides: {"brusselator_f"}
// Dependencies: {}
fn brusselator_f (x : f64 , y : f64 , t : f64) -> f64 { let eq1 = (x - 0.3) * (x - 0.3) + (y - 0.6) * (y - 0.6) <= 0.1 * 0.1 ; let eq2 = t >= 1.1 ; if eq1 && eq2 { 5.0 } else { 0.0 } }
};
}
