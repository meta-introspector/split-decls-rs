// Generated macro for arr_max (function)
macro_rules! Depcrate_safearr_max {
() => {
// Module: crate::safe
// Provides: {"arr_max"}
// Dependencies: {}
fn arr_max (n : usize , x : & [f64]) -> f64 { let mut max = f64 :: NEG_INFINITY ; for i in 0 .. n { if max < x [i] { max = x [i] ; } } max }
};
}
