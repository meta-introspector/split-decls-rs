// Generated macro for log_sum_exp (function)
macro_rules! Depcrate_safelog_sum_exp {
() => {
// Module: crate::safe
// Provides: {"log_sum_exp"}
// Dependencies: {}
fn log_sum_exp (n : usize , x : & [f64]) -> f64 { let mx = arr_max (n , x) ; let semx : f64 = x . iter () . map (| x | (x - mx) . exp ()) . sum () ; semx . ln () + mx }
};
}
