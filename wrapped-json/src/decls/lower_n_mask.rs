macro_rules! lower_n_mask {
    () => {
        # [doc = " Generate a bitwise mask for the lower `n` bits."] # [inline] pub (crate) fn lower_n_mask (n : u64) -> u64 { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! (n <= bits , "lower_n_mask() overflow in shl.") ; if n == bits { u64 :: MAX } else { (1 << n) - 1 } }
    };
}

lower_n_mask!();