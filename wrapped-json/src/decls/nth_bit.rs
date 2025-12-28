macro_rules! nth_bit {
    () => {
        # [doc = " Calculate a scalar factor of 2 above the halfway point."] # [inline] pub (crate) fn nth_bit (n : u64) -> u64 { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! (n < bits , "nth_bit() overflow in shl.") ; 1 << n }
    };
}

nth_bit!();