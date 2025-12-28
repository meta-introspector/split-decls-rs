macro_rules! deps {
    () => {
        ExtendedFloat!();
        Float!();
    };
}

macro_rules! avoid_overflow {
    () => {
        deps!();
        # [inline] pub (crate) fn avoid_overflow < F > (fp : & mut ExtendedFloat) where F : Float , { if fp . exp >= F :: MAX_EXPONENT { let diff = fp . exp - F :: MAX_EXPONENT ; if diff <= F :: MANTISSA_SIZE { let bit = (F :: MANTISSA_SIZE + 1) as u64 ; let n = (diff + 1) as u64 ; let mask = internal_n_mask (bit , n) ; if (fp . mant & mask) == 0 { let shift = diff + 1 ; shl (fp , shift) ; } } } }
    };
}

avoid_overflow!();