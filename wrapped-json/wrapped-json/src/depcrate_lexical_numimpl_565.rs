// Generated macro for impl_565 (impl)
macro_rules! Depcrate_lexical_numimpl_565 {
() => {
// Module: crate::lexical::num
// Provides: {"impl_565"}
// Dependencies: {}
impl Float for f32 { type Unsigned = u32 ; const ZERO : f32 = 0.0 ; const MAX_DIGITS : usize = 114 ; const EXPONENT_MASK : u32 = 0x7F800000 ; const HIDDEN_BIT_MASK : u32 = 0x00800000 ; const MANTISSA_MASK : u32 = 0x007FFFFF ; const INFINITY_BITS : u32 = 0x7F800000 ; const MANTISSA_SIZE : i32 = 23 ; const EXPONENT_BIAS : i32 = 127 + Self :: MANTISSA_SIZE ; const DENORMAL_EXPONENT : i32 = 1 - Self :: EXPONENT_BIAS ; const MAX_EXPONENT : i32 = 0xFF - Self :: EXPONENT_BIAS ; const DEFAULT_SHIFT : i32 = u64 :: FULL - f32 :: MANTISSA_SIZE - 1 ; const CARRY_MASK : u64 = 0x1000000 ; # [inline] fn exponent_limit () -> (i32 , i32) { (- 10 , 10) } # [inline] fn mantissa_limit () -> i32 { 7 } # [inline] fn pow10 (self , n : i32) -> f32 { debug_assert ! ({ let (min , max) = Self :: exponent_limit () ; n >= min && n <= max }) ; if n > 0 { self * F32_POW10 [n as usize] } else { self / F32_POW10 [- n as usize] } } # [inline] fn from_bits (u : u32) -> f32 { f32 :: from_bits (u) } # [inline] fn to_bits (self) -> u32 { f32 :: to_bits (self) } # [inline] fn is_sign_positive (self) -> bool { f32 :: is_sign_positive (self) } }
};
}
