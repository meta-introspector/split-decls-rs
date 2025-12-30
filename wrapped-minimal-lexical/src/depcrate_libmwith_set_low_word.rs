// Generated macro for with_set_low_word (function)
macro_rules! Depcrate_libmwith_set_low_word {
() => {
// Module: crate::libm
// Provides: {"with_set_low_word"}
// Dependencies: {}
# [inline] fn with_set_low_word (f : f64 , lo : u32) -> f64 { let mut tmp = f . to_bits () ; tmp &= 0xffffffff_00000000 ; tmp |= lo as u64 ; f64 :: from_bits (tmp) }
};
}
