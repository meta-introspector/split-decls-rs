macro_rules! deps {
    () => {
        FastPathRadix!();
    };
}

macro_rules! int_pow_fast_path {
    () => {
        deps!();
        # [doc = " Get a small, integral power-of-radix for fast-path multiplication."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Safe as long as the exponent is smaller than the table size."] # [inline (always)] pub (crate) unsafe fn int_pow_fast_path (exponent : usize , radix : FastPathRadix) -> u64 { # [cfg (not (feature = "compact"))] return match radix { FastPathRadix :: Five => unsafe { * SMALL_INT_POW5 . get_unchecked (exponent) } , FastPathRadix :: Ten => unsafe { * SMALL_INT_POW10 . get_unchecked (exponent) } , } ; # [cfg (feature = "compact")] return u64 :: from (radix) . pow (exponent as u32) ; }
    };
}

int_pow_fast_path!()