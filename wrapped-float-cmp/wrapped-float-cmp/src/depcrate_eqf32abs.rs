// Generated macro for f32abs (function)
macro_rules! Depcrate_eqf32abs {
() => {
// Module: crate::eq
// Provides: {"f32abs"}
// Dependencies: {}
# [inline (always)] fn f32abs (x : f32) -> f32 { f32 :: from_bits (x . to_bits () & ! (1 << 31)) }
};
}
