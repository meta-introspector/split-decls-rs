// Generated macro for f64_ordered_bits (function)
macro_rules! Depcrate_ulpsf64_ordered_bits {
() => {
// Module: crate::ulps
// Provides: {"f64_ordered_bits"}
// Dependencies: {}
# [inline] fn f64_ordered_bits (f : f64) -> u64 { const SIGN_BIT : u64 = 1 << 63 ; let bits = f . to_bits () ; if bits & SIGN_BIT != 0 { ! bits } else { bits ^ SIGN_BIT } }
};
}
