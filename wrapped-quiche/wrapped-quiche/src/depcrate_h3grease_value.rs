// Generated macro for grease_value (function)
macro_rules! Depcrate_h3grease_value {
() => {
// Module: crate::h3
// Provides: {"grease_value"}
// Dependencies: {}
# [doc = " Generates an HTTP/3 GREASE variable length integer."] pub fn grease_value () -> u64 { let n = super :: rand :: rand_u64_uniform (148_764_065_110_560_899) ; 31 * n + 33 }
};
}
