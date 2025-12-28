macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! compute_error {
    () => {
        deps!();
        # [doc = " Fallback algorithm to calculate the non-rounded representation."] # [doc = " This calculates the extended representation, and then normalizes"] # [doc = " the resulting representation, so the high bit is set."] # [inline] pub fn compute_error < F : Float > (q : i32 , mut w : u64) -> ExtendedFloat { let lz = w . leading_zeros () as i32 ; w <<= lz ; let hi = compute_product_approx (q , w , F :: MANTISSA_SIZE as usize + 3) . 1 ; compute_error_scaled :: < F > (q , hi , lz) }
    };
}

compute_error!();