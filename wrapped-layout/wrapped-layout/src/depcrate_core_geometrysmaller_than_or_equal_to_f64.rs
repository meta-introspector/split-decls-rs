// Generated macro for smaller_than_or_equal_to_f64 (function)
macro_rules! Depcrate_core_geometrysmaller_than_or_equal_to_f64 {
() => {
// Module: crate::core::geometry
// Provides: {"smaller_than_or_equal_to_f64"}
// Dependencies: {}
# [doc = " Similar to usual smaller than or equal to op, except for equal is withint f64 epsilon"] fn smaller_than_or_equal_to_f64 (x : f64 , y : f64) -> bool { if x > y { false } else { approx_eq_f64 (x , y) } }
};
}
