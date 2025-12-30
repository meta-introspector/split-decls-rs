// Generated macro for impl_122 (impl)
macro_rules! Depcrate_numimpl_122 {
() => {
// Module: crate::num
// Provides: {"impl_122"}
// Dependencies: {}
impl Float for f32 { const MAX_DIGITS : usize = 114 ; const SIGN_MASK : u64 = 0x80000000 ; const EXPONENT_MASK : u64 = 0x7F800000 ; const HIDDEN_BIT_MASK : u64 = 0x00800000 ; const MANTISSA_MASK : u64 = 0x007FFFFF ; const MANTISSA_SIZE : i32 = 23 ; const EXPONENT_BIAS : i32 = 127 + Self :: MANTISSA_SIZE ; const DENORMAL_EXPONENT : i32 = 1 - Self :: EXPONENT_BIAS ; const MAX_EXPONENT : i32 = 0xFF - Self :: EXPONENT_BIAS ; const CARRY_MASK : u64 = 0x1000000 ; const MIN_EXPONENT_ROUND_TO_EVEN : i32 = - 17 ; const MAX_EXPONENT_ROUND_TO_EVEN : i32 = 10 ; const MINIMUM_EXPONENT : i32 = - 127 ; const SMALLEST_POWER_OF_TEN : i32 = - 65 ; const LARGEST_POWER_OF_TEN : i32 = 38 ; const MIN_EXPONENT_FAST_PATH : i32 = - 10 ; const MAX_EXPONENT_FAST_PATH : i32 = 10 ; const MAX_EXPONENT_DISGUISED_FAST_PATH : i32 = 17 ; # [inline (always)] unsafe fn pow_fast_path (exponent : usize) -> Self { # [cfg (not (feature = "compact"))] return unsafe { * SMALL_F32_POW10 . get_unchecked (exponent) } ; # [cfg (feature = "compact")] return powf (10.0f32 , exponent as f32) ; } # [inline] fn from_u64 (u : u64) -> f32 { u as _ } # [inline] fn from_bits (u : u64) -> f32 { debug_assert ! (u <= 0xffff_ffff) ; f32 :: from_bits (u as u32) } # [inline] fn to_bits (self) -> u64 { f32 :: to_bits (self) as u64 } }
};
}
