// Generated macro for normalize_float (function)
macro_rules! Depcrate_colornormalize_float {
() => {
// Module: crate::color
// Provides: {"normalize_float"}
// Dependencies: {}
# [inline] fn normalize_float (float : f32 , max : f32) -> f32 { # [allow (clippy :: neg_cmp_op_on_partial_ord)] let clamped = if ! (float < 1.0) { 1.0 } else { float . max (0.0) } ; (clamped * max) . round () }
};
}
