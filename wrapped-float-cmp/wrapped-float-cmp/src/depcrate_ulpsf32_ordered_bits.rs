// Generated macro for f32_ordered_bits (function)
macro_rules! Depcrate_ulpsf32_ordered_bits {
() => {
// Module: crate::ulps
// Provides: {"f32_ordered_bits"}
// Dependencies: {}
# [inline] fn f32_ordered_bits (f : f32) -> u32 { const SIGN_BIT : u32 = 1 << 31 ; let bits = f . to_bits () ; if bits & SIGN_BIT != 0 { ! bits } else { bits ^ SIGN_BIT } }
};
}
