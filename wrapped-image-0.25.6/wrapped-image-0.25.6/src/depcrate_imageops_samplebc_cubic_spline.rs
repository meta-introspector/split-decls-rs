// Generated macro for bc_cubic_spline (function)
macro_rules! Depcrate_imageops_samplebc_cubic_spline {
() => {
// Module: crate::imageops::sample
// Provides: {"bc_cubic_spline"}
// Dependencies: {}
fn bc_cubic_spline (x : f32 , b : f32 , c : f32) -> f32 { let a = x . abs () ; let k = if a < 1.0 { (12.0 - 9.0 * b - 6.0 * c) * a . powi (3) + (- 18.0 + 12.0 * b + 6.0 * c) * a . powi (2) + (6.0 - 2.0 * b) } else if a < 2.0 { (- b - 6.0 * c) * a . powi (3) + (6.0 * b + 30.0 * c) * a . powi (2) + (- 12.0 * b - 48.0 * c) * a + (8.0 * b + 24.0 * c) } else { 0.0 } ; k / 6.0 }
};
}
