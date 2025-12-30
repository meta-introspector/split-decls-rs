// Generated macro for r (function)
macro_rules! Depcrate_math_asinfr {
() => {
// Module: crate::math::asinf
// Provides: {"r"}
// Dependencies: {}
fn r (z : f32) -> f32 { let p = z * (P_S0 + z * (P_S1 + z * P_S2)) ; let q = 1. + z * Q_S1 ; p / q }
};
}
