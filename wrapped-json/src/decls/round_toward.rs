macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! round_toward {
    () => {
        deps!();
        # [inline] fn round_toward (fp : & mut ExtendedFloat , shift : i32) -> bool { let mask : u64 = lower_n_mask (shift as u64) ; let truncated_bits = fp . mant & mask ; overflowing_shr (fp , shift) ; truncated_bits != 0 }
    };
}

round_toward!();