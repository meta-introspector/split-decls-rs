// Generated macro for approx_eq_f64 (function)
macro_rules! Depcrate_core_geometryapprox_eq_f64 {
() => {
// Module: crate::core::geometry
// Provides: {"approx_eq_f64"}
// Dependencies: {}
# [doc = " trivial function for checking aproximate equality of f64, within epsion of f64"] fn approx_eq_f64 (x : f64 , y : f64) -> bool { if x == 0. { y . abs () < f64 :: EPSILON } else if y == 0. { x . abs () < f64 :: EPSILON } else { let abs_diff = (x - y) . abs () ; if abs_diff < f64 :: EPSILON { true } else { abs_diff / x . abs () . max (y . abs ()) < f64 :: EPSILON } } }
};
}
