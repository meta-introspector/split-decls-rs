macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! error_is_accurate {
    () => {
        deps!();
        # [doc = " Determine if the number of errors is tolerable for float precision."] fn error_is_accurate < F : Float > (errors : u32 , fp : & ExtendedFloat) -> bool { debug_assert ! (fp . exp >= - 64) ; let mantissa_shift = 64 - F :: MANTISSA_SIZE - 1 ; let extrabits = match fp . exp <= - mantissa_shift { true => 1 - fp . exp , false => 64 - F :: MANTISSA_SIZE - 1 , } ; let maskbits = extrabits as u64 ; let errors = errors as u64 ; if extrabits > 64 { ! fp . mant . overflowing_add (errors) . 1 } else { let mask = lower_n_mask (maskbits) ; let extra = fp . mant & mask ; let halfway = lower_n_halfway (maskbits) ; let cmp1 = halfway . wrapping_sub (errors) < extra ; let cmp2 = extra < halfway . wrapping_add (errors) ; ! (cmp1 && cmp2) } }
    };
}

error_is_accurate!()