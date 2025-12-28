macro_rules! u128_ilog2 {
    () => {
        # [doc = " `u128::ilog2`"] const fn u128_ilog2 (v : u128) -> u32 { assert ! (v != 0) ; u128 :: BITS - 1 - v . leading_zeros () }
    };
}

u128_ilog2!();