// Generated macro for f64abs (function)
macro_rules! Depcrate_eqf64abs {
() => {
// Module: crate::eq
// Provides: {"f64abs"}
// Dependencies: {}
# [inline (always)] fn f64abs (x : f64) -> f64 { f64 :: from_bits (x . to_bits () & ! (1 << 63)) }
};
}
