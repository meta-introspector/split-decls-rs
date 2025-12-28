macro_rules! compute_product_approx {
    () => {
        fn compute_product_approx (q : i32 , w : u64 , precision : usize) -> (u64 , u64) { debug_assert ! (q >= SMALLEST_POWER_OF_FIVE) ; debug_assert ! (q <= LARGEST_POWER_OF_FIVE) ; debug_assert ! (precision <= 64) ; let mask = if precision < 64 { 0xFFFF_FFFF_FFFF_FFFF_u64 >> precision } else { 0xFFFF_FFFF_FFFF_FFFF_u64 } ; let index = (q - SMALLEST_POWER_OF_FIVE) as usize ; let (lo5 , hi5) = POWER_OF_FIVE_128 [index] ; let (mut first_lo , mut first_hi) = full_multiplication (w , lo5) ; if first_hi & mask == mask { let (_ , second_hi) = full_multiplication (w , hi5) ; first_lo = first_lo . wrapping_add (second_hi) ; if second_hi > first_lo { first_hi += 1 ; } } (first_lo , first_hi) }
    };
}

compute_product_approx!()