macro_rules! u64_to_hi64_1 {
    () => {
        # [doc = " Shift 64-bit integer to high 64-bits."] # [inline] fn u64_to_hi64_1 (r0 : u64) -> (u64 , bool) { debug_assert ! (r0 != 0) ; let ls = r0 . leading_zeros () ; (r0 << ls , false) }
    };
}

u64_to_hi64_1!()