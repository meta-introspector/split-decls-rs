// Generated macro for near_zero (function)
macro_rules! Depcratenear_zero {
() => {
// Module: crate
// Provides: {"near_zero"}
// Dependencies: {}
fn near_zero (value : f64) -> bool { const EPS : f64 = 1E-8 ; if value < 0.0 { - value < EPS } else { value < EPS } }
};
}
