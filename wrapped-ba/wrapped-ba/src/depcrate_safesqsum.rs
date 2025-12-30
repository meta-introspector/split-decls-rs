// Generated macro for sqsum (function)
macro_rules! Depcrate_safesqsum {
() => {
// Module: crate::safe
// Provides: {"sqsum"}
// Dependencies: {}
fn sqsum (x : & [f64]) -> f64 { x . iter () . map (| & v | v * v) . sum () }
};
}
