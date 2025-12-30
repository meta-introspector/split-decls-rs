// Generated macro for impl_627 (impl)
macro_rules! Depcrate_lexical_numimpl_627 {
() => {
// Module: crate::lexical::num
// Provides: {"impl_627"}
// Dependencies: {}
impl Float for f64 { type Unsigned = u64 ; const ZERO : f64 = 0.0 ; const MAX_DIGITS : usize = 769 ; const EXPONENT_MASK : u64 = 0x7FF0000000000000 ; const HIDDEN_BIT_MASK : u64 = 0x0010000000000000 ; const MANTISSA_MASK : u64 = 0x000FFFFFFFFFFFFF ; const INFINITY_BITS : u64 = 0x7FF0000000000000 ; const MANTISSA_SIZE : i32 = 52 ; const EXPONENT_BIAS : i32 = 1023 + Self :: MANTISSA_SIZE ; const DENORMAL_EXPONENT : i32 = 1 - Self :: EXPONENT_BIAS ; const MAX_EXPONENT : i32 = 0x7FF - Self :: EXPONENT_BIAS ; const DEFAULT_SHIFT : i32 = u64 :: FULL - f64 :: MANTISSA_SIZE - 1 ; const CARRY_MASK : u64 = 0x20000000000000 ; # [inline] fn exponent_limit () -> (i32 , i32) { (- 22 , 22) } # [inline] fn mantissa_limit () -> i32 { 15 } # [inline] fn pow10 (self , n : i32) -> f64 { debug_assert ! ({ let (min , max) = Self :: exponent_limit () ; n >= min && n <= max }) ; if n > 0 { self * F64_POW10 [n as usize] } else { self / F64_POW10 [- n as usize] } } # [inline] fn from_bits (u : u64) -> f64 { f64 :: from_bits (u) } # [inline] fn to_bits (self) -> u64 { f64 :: to_bits (self) } # [inline] fn is_sign_positive (self) -> bool { f64 :: is_sign_positive (self) } }
};
}
