// Generated macro for sqnorm (function)
macro_rules! Depcrate_safesqnorm {
() => {
// Module: crate::safe
// Provides: {"sqnorm"}
// Dependencies: {}
fn sqnorm (x : & [f64]) -> f64 { x . iter () . map (| x | x * x) . sum () }
};
}
