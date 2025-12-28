macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! round_nearest_tie_even {
    () => {
        deps!();
        # [doc = " Shift right N-bytes and round towards a direction."] # [doc = ""] # [doc = " Callback should take the following parameters:"] # [doc = "     1. is_odd"] # [doc = "     1. is_halfway"] # [doc = "     1. is_above"] # [cfg_attr (not (feature = "compact") , inline)] pub fn round_nearest_tie_even < Cb > (fp : & mut ExtendedFloat , shift : i32 , cb : Cb) where Cb : Fn (bool , bool , bool) -> bool , { debug_assert ! (shift <= 64) ; let mask = lower_n_mask (shift as u64) ; let halfway = lower_n_halfway (shift as u64) ; let truncated_bits = fp . mant & mask ; let is_above = truncated_bits > halfway ; let is_halfway = truncated_bits == halfway ; fp . mant = match shift == 64 { true => 0 , false => fp . mant >> shift , } ; fp . exp += shift ; let is_odd = fp . mant & 1 == 1 ; fp . mant += cb (is_odd , is_halfway , is_above) as u64 ; }
    };
}

round_nearest_tie_even!();