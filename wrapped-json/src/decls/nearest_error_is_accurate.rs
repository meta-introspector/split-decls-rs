macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! nearest_error_is_accurate {
    () => {
        deps!();
        # [doc = " Check if the error is accurate with a round-nearest rounding scheme."] # [inline] fn nearest_error_is_accurate (errors : u64 , fp : & ExtendedFloat , extrabits : u64) -> bool { if extrabits == 65 { ! fp . mant . overflowing_add (errors) . 1 } else { let mask : u64 = lower_n_mask (extrabits) ; let extra : u64 = fp . mant & mask ; let halfway : u64 = lower_n_halfway (extrabits) ; let cmp1 = halfway . wrapping_sub (errors) < extra ; let cmp2 = extra < halfway . wrapping_add (errors) ; ! (cmp1 && cmp2) } }
    };
}

nearest_error_is_accurate!()