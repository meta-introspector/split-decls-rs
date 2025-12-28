macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Float for f64 { const MAX_DIGITS : usize = 769 ; const SIGN_MASK : u64 = 0x8000000000000000 ; const EXPONENT_MASK : u64 = 0x7FF0000000000000 ; const HIDDEN_BIT_MASK : u64 = 0x0010000000000000 ; const MANTISSA_MASK : u64 = 0x000FFFFFFFFFFFFF ; const MANTISSA_SIZE : i32 = 52 ; const EXPONENT_BIAS : i32 = 1023 + Self :: MANTISSA_SIZE ; const DENORMAL_EXPONENT : i32 = 1 - Self :: EXPONENT_BIAS ; const MAX_EXPONENT : i32 = 0x7FF - Self :: EXPONENT_BIAS ; const CARRY_MASK : u64 = 0x20000000000000 ; const MIN_EXPONENT_ROUND_TO_EVEN : i32 = - 4 ; const MAX_EXPONENT_ROUND_TO_EVEN : i32 = 23 ; const MINIMUM_EXPONENT : i32 = - 1023 ; const SMALLEST_POWER_OF_TEN : i32 = - 342 ; const LARGEST_POWER_OF_TEN : i32 = 308 ; const MIN_EXPONENT_FAST_PATH : i32 = - 22 ; const MAX_EXPONENT_FAST_PATH : i32 = 22 ; const MAX_EXPONENT_DISGUISED_FAST_PATH : i32 = 37 ; # [inline (always)] unsafe fn pow_fast_path (exponent : usize) -> Self { # [cfg (not (feature = "compact"))] return unsafe { * SMALL_F64_POW10 . get_unchecked (exponent) } ; # [cfg (feature = "compact")] return powd (10.0f64 , exponent as f64) ; } # [inline] fn from_u64 (u : u64) -> f64 { u as _ } # [inline] fn from_bits (u : u64) -> f64 { f64 :: from_bits (u) } # [inline] fn to_bits (self) -> u64 { f64 :: to_bits (self) } }
    };
}

impl_97!()