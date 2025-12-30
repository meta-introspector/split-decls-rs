// Generated macro for normalize_scale_vector (function)
macro_rules! Depcrate_core_geometrynormalize_scale_vector {
() => {
// Module: crate::core::geometry
// Provides: {"normalize_scale_vector"}
// Dependencies: {}
# [doc = " Return the normalized vector \\p v multiplied by the scalar \\p s."] pub fn normalize_scale_vector (v : Point , s : f64) -> Point { let len = Point :: zero () . distance_to (v) ; assert ! (len > 0. , "Can't normalize the unit vector") ; v . scale (s / len) }
};
}
