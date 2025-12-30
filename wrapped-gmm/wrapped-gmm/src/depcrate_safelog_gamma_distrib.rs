// Generated macro for log_gamma_distrib (function)
macro_rules! Depcrate_safelog_gamma_distrib {
() => {
// Module: crate::safe
// Provides: {"log_gamma_distrib"}
// Dependencies: {}
fn log_gamma_distrib (a : f64 , p : f64) -> f64 { 0.25 * p * (p - 1.) * PI . ln () + (1 ..= p as usize) . map (| j | lgamma (a + 0.5 * (1. - j as f64))) . sum :: < f64 > () }
};
}
