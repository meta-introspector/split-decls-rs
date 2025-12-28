macro_rules! internal_n_mask {
    () => {
        # [doc = " Calculate a bitwise mask with `n` 1 bits starting at the `bit` position."] # [inline] pub (crate) fn internal_n_mask (bit : u64 , n : u64) -> u64 { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! (bit <= bits , "internal_n_halfway() overflow in shl.") ; debug_assert ! (n <= bits , "internal_n_halfway() overflow in shl.") ; debug_assert ! (bit >= n , "internal_n_halfway() overflow in sub.") ; lower_n_mask (bit) ^ lower_n_mask (bit - n) }
    };
}

internal_n_mask!()