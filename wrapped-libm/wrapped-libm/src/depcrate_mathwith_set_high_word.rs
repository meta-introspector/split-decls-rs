// Generated macro for with_set_high_word (function)
macro_rules! Depcrate_mathwith_set_high_word {
() => {
// Module: crate::math
// Provides: {"with_set_high_word"}
// Dependencies: {}
# [inline] fn with_set_high_word (f : f64 , hi : u32) -> f64 { let mut tmp = f . to_bits () ; tmp &= 0x00000000_ffffffff ; tmp |= (hi as u64) << 32 ; f64 :: from_bits (tmp) }
};
}
