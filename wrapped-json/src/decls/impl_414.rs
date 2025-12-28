macro_rules! deps {
    () => {
        ExtendedFloat!();
        Float!();
        FloatErrors!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl FloatErrors for u64 { # [inline] fn error_scale () -> u32 { 8 } # [inline] fn error_halfscale () -> u32 { u64 :: error_scale () / 2 } # [inline] fn error_is_accurate < F : Float > (count : u32 , fp : & ExtendedFloat) -> bool { let bias = - (F :: EXPONENT_BIAS - F :: MANTISSA_SIZE) ; let denormal_exp = bias - 63 ; let extrabits = if fp . exp <= denormal_exp { 64 - F :: MANTISSA_SIZE + denormal_exp - fp . exp } else { 63 - F :: MANTISSA_SIZE } ; let extrabits = extrabits as u64 ; let errors = count as u64 ; if extrabits > 65 { return true ; } nearest_error_is_accurate (errors , fp , extrabits) } }
    };
}

impl_414!();