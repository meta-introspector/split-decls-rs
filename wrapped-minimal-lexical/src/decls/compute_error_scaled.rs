macro_rules! deps {
    () => {
        ExtendedFloat!();
        Float!();
    };
}

macro_rules! compute_error_scaled {
    () => {
        deps!();
        # [doc = " Compute the error from a mantissa scaled to the exponent."] # [inline] pub fn compute_error_scaled < F : Float > (q : i32 , mut w : u64 , lz : i32) -> ExtendedFloat { let hilz = (w >> 63) as i32 ^ 1 ; w <<= hilz ; let power2 = power (q as i32) + F :: EXPONENT_BIAS - hilz - lz - 62 ; ExtendedFloat { mant : w , exp : power2 + F :: INVALID_FP , } }
    };
}

compute_error_scaled!()